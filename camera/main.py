import subprocess
import time
import threading
import os
from math import sqrt

import numpy as np
import cv2
import dlib
 
DISTANCE_THRESHOLD = 500

class Runtime:
    num_of_faces = 0
    mutex = threading.Lock()

    application_name = "Enchiridion"

    window_name = "camera"
    window_width = 600
    window_height = 450

    def __init__(self):
        pass

    def run(self):
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
        detector = dlib.get_frontal_face_detector()

        detected_faces = []

        while True:
            self.mutex.acquire()

            ret, frame = cap.read()
            frame = cv2.flip(frame, 1)
         
            gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
            faces = detector(gray)

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
            cv2.resizeWindow(self.window_name, self.window_width, self.window_height)
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
   
        gstreamer_cmd = 'gst-launch-1.0 ximagesrc xid={pid} ! videoconvert ! x264enc bitrate=1000 tune=zerolatency ! video/x-h264 ! h264parse ! video/x-h264 ! queue ! flvmux name=muxer ! rtmpsink location="rtmp://18.143.23.68/live/livestream/1 live=1"'.format(pid=camera_frame_pid)

        with open(os.devnull, 'w') as fp:
            gst = subprocess.Popen(gstreamer_cmd, shell=True, stdout=fp)
            gst.wait()


    def timeseries(self):
        while True:
            time.sleep(1)

            self.mutex.acquire()
            print("num_of_faces ", self.num_of_faces, flush=True)
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
                    gst = subprocess.Popen(focus_cmd, shell=True, stdout=fp)
                    gst.wait()

                    is_focused = True
            except:
                print("Unable to find window with the name of {application_name}".format(application_name=self.application_name), flush=True)


rt = Runtime()
rt.run()
