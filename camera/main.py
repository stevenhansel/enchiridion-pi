import subprocess
import time
import threading
import os
import datetime
from math import sqrt

import argparse
import numpy as np
import cv2
import dlib
 
DISTANCE_THRESHOLD = 250

class Runtime:
    num_of_faces = 0
    mutex = threading.Lock()

    application_name = "Enchiridion"

    window_name = "camera"

    window_width = 1280
    window_height = 720

    def __init__(self, device_id, srs_ip):
        self.device_id = device_id
        self.srs_ip = srs_ip

    def run(self):
        # threading.Thread(target=self.timeseries).start()
        # self.camera()

        threads = [
            threading.Thread(target=self.camera),
            threading.Thread(target=self.gstreamer),
            threading.Thread(target=self.timeseries),
            threading.Thread(target=self.wmctrl_focuser)
        ]

        for thread in threads:
            thread.start()

    def camera(self):
        cap = cv2.VideoCapture(0)

        # cap.set(cv2.CAP_PROP_FRAME_WIDTH, self.window_width)
        # cap.set(cv2.CAP_PROP_FRAME_HEIGHT, self.window_height)

        detector = dlib.get_frontal_face_detector()

        while True:
            ret, frame = cap.read()

            frame = cv2.flip(frame, 1)
            gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

            faces = detector(frame)

            self.mutex.acquire()
            self.num_of_faces = len(faces)
            self.mutex.release()

            for i, face in enumerate(faces):   
                cv2.rectangle(frame, (face.left(), face.top()), (face.right(), face.bottom()), (0, 255, 0), 2)
                cv2.putText(frame, 'face '+str(i+1), (face.left() - 10, face.bottom() -10),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.7, (0, 0, 255), 2)
                    
            cv2.imshow(self.window_name, frame)
            cv2.setWindowProperty(self.window_name, cv2.WND_PROP_TOPMOST, 0)

            if cv2.waitKey(1) & 0xFF == ord('q'):
                break
     
        cap.release()
        cv2.destroyAllWindows()
   
    def gstreamer(self):
        camera_frame_pid = None

        while True:
            time.sleep(1)

            if camera_frame_pid == None:
                try:
                    wmctrl = subprocess.Popen(('wmctrl', '-lp'), stdout=subprocess.PIPE)
                    output = subprocess.check_output(('grep', self.window_name), stdin=wmctrl.stdout, text=True)

                    wmctrl.wait()

                    pid = output.split(' ')[0]
                    camera_frame_pid = pid

                    break
                except:
                    print("Camera frame not detected, trying again...", flush=True)

                    continue


        xdotool = subprocess.Popen(("xdotool", "windowmove", pid, "50", "100"), stdout=subprocess.PIPE)
        xdotool.wait()

        # gstreamer_cmd = '''gst-launch-1.0 ximagesrc xid={pid} ! videoscale ! 'video/x-raw,width={width},height={height}' ! videoconvert ! x264enc speed-preset=ultrafast tune=zerolatency byte-stream=true ! queue ! flvmux name=muxer ! rtmpsink location="rtmp://{srs_ip}/live/livestream/{device_id} live=1"'''.format(pid=camera_frame_pid, srs_ip=self.srs_ip, device_id=self.device_id, width=self.window_width, height=self.window_height)

        gstreamer_cmd = '''gst-launch-1.0 ximagesrc xid={pid} ! videoconvert ! x264enc speed-preset=ultrafast tune=zerolatency byte-stream=true ! queue ! flvmux name=muxer ! rtmpsink location="rtmp://{srs_ip}/live/livestream/{device_id} live=1"'''.format(pid=camera_frame_pid, srs_ip=self.srs_ip, device_id=self.device_id)

        with open(os.devnull, 'w') as fp:
            gst = subprocess.Popen(gstreamer_cmd, shell=True, stdout=fp)
            gst.wait()

    def timeseries(self):
        while True:
            time.sleep(1)

            self.mutex.acquire()
            timestamp = datetime.datetime.now(datetime.timezone.utc)

            print("{timestamp} {device_id} {num_of_faces}".format(timestamp=timestamp.isoformat(), device_id=self.device_id, num_of_faces=self.num_of_faces), flush=True)

            self.mutex.release()

    def wmctrl_focuser(self):
        while True:
            time.sleep(1)
            
            try:
                active_window = subprocess.check_output(("xdotool", "getwindowfocus", "getwindowname"), text=True).strip()
                if active_window != "camera":
                    continue

                wmctrl = subprocess.Popen(("wmctrl", "-lp"), stdout=subprocess.PIPE)
                output = subprocess.check_output(("grep", self.application_name), stdin=wmctrl.stdout, text=True)
                wmctrl.wait()

                if output == "":
                    continue

                focus_cmd = "wmctrl -a {application_name}".format(application_name=self.application_name)
                with open(os.devnull, 'w') as fp:
                    ps = subprocess.Popen(focus_cmd, shell=True, stdout=fp)
                    ps.wait()
            except:
                print("Unable to find window with the name of {application_name}".format(application_name=self.application_name), flush=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-id')
    parser.add_argument('-ip')

    args = parser.parse_args()

    rt = Runtime(args.id, args.ip)
    rt.run()

main()
