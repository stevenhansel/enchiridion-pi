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
    window_width = 800
    window_height = 600

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

        cap.set(cv2.CAP_PROP_FRAME_WIDTH, self.window_width)
        cap.set(cv2.CAP_PROP_FRAME_HEIGHT, self.window_height)

        detector = dlib.get_frontal_face_detector()

        detected_faces = []

        while True:
            ret, frame = cap.read()
            frame = cv2.flip(frame, 1)
            gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
            faces = detector(frame)

            self.mutex.acquire()

            if len(faces) == 0:
                detected_faces = []
            else:
                num_of_faces_difference = len(faces) - self.num_of_faces
                for i in range(len(faces)):
                    x1_midpoint = faces[i].left() + faces[i].right() / 2
                    y1_midpoint = faces[i].top() + faces[i].bottom() / 2

                    if num_of_faces_difference == 0:
                        for j, prev_face in enumerate(detected_faces):
                            x2_midpoint = prev_face["x1"] + prev_face["x2"] / 2
                            y2_midpoint = prev_face["y1"] + prev_face["y2"] / 2

                            distance = sqrt(pow(x2_midpoint - x1_midpoint, 2) + pow(y2_midpoint - y1_midpoint, 2)) 
                            if distance < DISTANCE_THRESHOLD:
                                detected_faces[j]["x1"] = faces[i].left()
                                detected_faces[j]["x2"] = faces[i].right()
                                detected_faces[j]["y1"] = faces[i].top()
                                detected_faces[j]["y2"] = faces[i].bottom()
                    elif num_of_faces_difference > 0:
                        if self.num_of_faces == 0:
                            detected_faces.append({
                                "x1": faces[i].left(),
                                "y1": faces[i].top(),
                                "x2": faces[i].right(),
                                "y2": faces[i].bottom(),
                            })
                        else:
                            for j, prev_face in enumerate(detected_faces):
                                x2_midpoint = prev_face["x1"] + prev_face["x2"] / 2
                                y2_midpoint = prev_face["y1"] + prev_face["y2"] / 2

                                distance = sqrt(pow(x2_midpoint - x1_midpoint, 2) + pow(y2_midpoint - y1_midpoint, 2))

                                if distance < DISTANCE_THRESHOLD:
                                    detected_faces[j]["x1"] = faces[i].left()
                                    detected_faces[j]["x2"] = faces[i].right()
                                    detected_faces[j]["y1"] = faces[i].top()
                                    detected_faces[j]["y2"] = faces[i].bottom()
                                else:
                                    detected_faces.append({
                                        "x1": faces[i].left(),
                                        "y1": faces[i].top(),
                                        "x2": faces[i].right(),
                                        "y2": faces[i].bottom(),
                                    })
                    else:
                            for j, prev_face in enumerate(detected_faces):
                                x2_midpoint = prev_face["x1"] + prev_face["x2"] / 2
                                y2_midpoint = prev_face["y1"] + prev_face["y2"] / 2

                                distance = sqrt(pow(x2_midpoint - x1_midpoint, 2) + pow(y2_midpoint - y1_midpoint, 2))

                                if distance < DISTANCE_THRESHOLD:
                                    detected_faces[j]["x1"] = faces[i].left()
                                    detected_faces[j]["x2"] = faces[i].right()
                                    detected_faces[j]["y1"] = faces[i].top()
                                    detected_faces[j]["y2"] = faces[i].bottom()
                                else:
                                    detected_faces.pop(j)

            for i, face in enumerate(detected_faces):   
                cv2.rectangle(frame, (face["x1"], face["y1"]), (face["x2"], face["y2"]), (0, 255, 0), 2)
                cv2.putText(frame, 'face '+str(i+1), (face["x1"] - 10, face["y2"] -10),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.7, (0, 0, 255), 2)
                    
            self.num_of_faces = len(detected_faces)
            self.mutex.release()

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
   
        # gstreamer_cmd = '''gst-launch-1.0 ximagesrc xid={pid} ! videoscale ! 'video/x-raw,width=600,height=450' ! videoconvert ! x264enc bitrate=1000 tune=zerolatency ! h264parse ! queue ! flvmux name=muxer ! rtmpsink location="rtmp://{srs_ip}/live/livestream live=1"'''.format(pid=camera_frame_pid, srs_ip=self.srs_ip, device_id=self.device_id)

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
