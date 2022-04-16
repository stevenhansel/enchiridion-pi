package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/signal"
	"path"
	"syscall"
	"time"

	"github.com/adjust/rmq/v4"
	"github.com/zealic/go2node"
)

const (
	prefetchLimit = 1000
	pollDuration  = 100 * time.Millisecond
	numConsumers  = 5

	reportBatchSize = 10000
	consumeDuration = time.Millisecond
	shouldLog       = true
)

type SyncJobPayload struct {
	Operation string `json:"operation"`
	URL       string `json:"imageUrl"`
	Filename  string `json:"filename"`
}

func getSyncQueueName(deviceID int) string {
	return fmt.Sprintf("sync-device-%d", deviceID)
}

// TODO: refactor the consumer function (this is from example)
func main() {
	channel, err := go2node.RunAsNodeChild()
	if err != nil {
		fmt.Fprintf(os.Stdout, "err: failed to run as node child %v", err)
		os.Exit(1)
	}

	channel.Write(&go2node.NodeMessage{
		Message: []byte(`{"status":"success"}`),
	})

	var localImagePath string
	flag.StringVar(&localImagePath, "path", "", "")

	var redisQueueAddr string
	flag.StringVar(&redisQueueAddr, "redis", "", "")

	flag.Parse()

	if localImagePath == "" {
		fmt.Fprintf(os.Stdout, "err: local image path is empty")
		os.Exit(1)
	}

	if redisQueueAddr == "" {
		fmt.Fprintf(os.Stdout, "err: redis queue addr is empty")
		os.Exit(1)
	}

	fmt.Println("redis queue addr: ", redisQueueAddr)

	errChan := make(chan error, 10)
	go logErrors(errChan)

	connection, err := rmq.OpenConnection("consumer", "tcp", redisQueueAddr, 1, errChan)
	if err != nil {
		panic(err)
	}

	fmt.Println("starting...")

	// TODO: Sync Queue Name should be from flag parameter
	queue, err := connection.OpenQueue(getSyncQueueName(1))
	if err != nil {
		panic(err)
	}

	if err := queue.StartConsuming(prefetchLimit, pollDuration); err != nil {
		panic(err)
	}

	for i := 0; i < numConsumers; i++ {
		name := fmt.Sprintf("consumer %d", i)
		if _, err := queue.AddConsumer(name, NewConsumer(i, localImagePath, channel)); err != nil {
			panic(err)
		}
	}

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, syscall.SIGINT)
	defer signal.Stop(signals)

	<-signals // wait for signal
	go func() {
		<-signals // hard exit on second signal (in case shutdown gets stuck)
		os.Exit(1)
	}()

	<-connection.StopAllConsuming() // wait for all Consume() calls to finish
}

type Consumer struct {
	name           string
	count          int
	before         time.Time
	localImagePath string
	channel        go2node.NodeChannel
}

func NewConsumer(tag int, path string, channel go2node.NodeChannel) *Consumer {
	return &Consumer{
		name:           fmt.Sprintf("consumer%d", tag),
		count:          0,
		before:         time.Now(),
		localImagePath: path,
		channel:        channel,
	}
}

func (consumer *Consumer) Consume(delivery rmq.Delivery) {
	payload := delivery.Payload()
	debugf("start consume %s", payload)

	data := &SyncJobPayload{}
	err := json.Unmarshal([]byte(payload), data)
	if err != nil {
		debugf("unmarshalling failed")
		rejectDelivery(delivery)
	}

	if data.Operation == "append" {
		err = consumer.processImage(data)
		if err != nil {
			debugf("processing image failed, err: %v", err)
			rejectDelivery(delivery)
		}
	} else if data.Operation == "delete" {
		err = consumer.deleteImage(data)
		if err != nil {
			debugf("deleting image failed, err: %v", err)
			rejectDelivery(delivery)
		}
	}

	consumer.count++
	if consumer.count%reportBatchSize == 0 {
		duration := time.Now().Sub(consumer.before)
		consumer.before = time.Now()
		perSecond := time.Second / (duration / reportBatchSize)
		log.Printf("%s consumed %d %s %d", consumer.name, consumer.count, payload, perSecond)
	}

	if consumer.count%reportBatchSize > 0 {
		if err := delivery.Ack(); err != nil {
			debugf("failed to ack %s: %s", payload, err)
		} else {
			debugf("acked %s", payload)
		}
	} else { // reject one per batch
		rejectDelivery(delivery)
	}
}

func rejectDelivery(delivery rmq.Delivery) {
	if err := delivery.Reject(); err != nil {
		debugf("failed to reject %s", err)
	} else {
		debugf("rejected successfully")
	}
}

func logErrors(errChan <-chan error) {
	for err := range errChan {
		switch err := err.(type) {
		case *rmq.HeartbeatError:
			if err.Count == rmq.HeartbeatErrorLimit {
				log.Print("heartbeat error (limit): ", err)
			} else {
				log.Print("heartbeat error: ", err)
			}
		case *rmq.ConsumeError:
			log.Print("consume error: ", err)
		case *rmq.DeliveryError:
			log.Print("delivery error: ", err.Delivery, err)
		default:
			log.Print("other error: ", err)
		}
	}
}

func debugf(format string, args ...interface{}) {
	if shouldLog {
		log.Printf(format, args...)
	}
}

func (c *Consumer) downloadFile(url, filename string) error {
	tmp := path.Join(c.localImagePath, filename+".tmp")
	file, err := os.Create(tmp)
	if err != nil {
		return err
	}

	defer file.Close()

	res, err := http.Get(url)
	if err != nil {
		return err
	}

	defer res.Body.Close()

	_, err = io.Copy(file, res.Body)
	if err != nil {
		return err
	}

	err = os.Rename(tmp, path.Join(c.localImagePath, filename))
	if err != nil {
		return err
	}

	return nil
}

func (c *Consumer) processImage(data *SyncJobPayload) error {
	if err := c.downloadFile(data.URL, data.Filename); err != nil {
		return err
	}

	c.channel.Write(&go2node.NodeMessage{
		Message: []byte(`{"status":"success"}`),
	})
	return nil
}

func (c *Consumer) deleteImage(data *SyncJobPayload) error {
	err := os.Remove(path.Join(c.localImagePath, data.Filename))
	if err != nil {
		return err
	}

	c.channel.Write(&go2node.NodeMessage{
		Message: []byte(`{"status":"success"}`),
	})

	return nil
}
