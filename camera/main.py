import os
from math import sqrt
import numpy as np
import cv2
import dlib
 
DISTANCE_THRESHOLD = 250 

# Connects to your computer's default camera
cap = cv2.VideoCapture(0)
 
# Detect the coordinates
detector = dlib.get_frontal_face_detector()

num_of_faces = 0
detected_faces = []

# Capture frames continuously
while True:
    # Capture frame-by-frame
    ret, frame = cap.read()
    frame = cv2.flip(frame, 1)
 
    # RGB to grayscale
    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    faces = detector(gray)

    num_of_faces_difference = len(faces) - num_of_faces
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
            if num_of_faces == 0:
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
        # Display the box and faces
        cv2.putText(frame, 'face num'+str(i), (face["x1"] - 10, face["y2"] -10),
                    cv2.FONT_HERSHEY_SIMPLEX, 0.7, (0, 0, 255), 2)
        
    num_of_faces = len(detected_faces)
    cv2.imshow('camera', frame)
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break
 
 
# Release the capture and destroy the windows
cap.release()
cv2.destroyAllWindows()
