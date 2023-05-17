from random import random
import pandas
from .math import (
    Vector,
    wrap_around,
    delta,
    normalize_2,
    cross,
    angle,
)
from .common import logging
import math


f32 = float
MovementAction = str


class Target():
    def __init__(self):
        aa = 0.0001
        self.p = Vector(random(),random())
        self.pp = Vector( 
            self.p.x + random()*aa - aa*0.5,
            self.p.y + random()*aa - aa*0.5,
        )


class Ship():
    def __init__(self):
        aa = 0.0001
        self.p = Vector(random(),random())
        self.pp = Vector( 
            self.p.x + random()*aa - aa*0.5,
            self.p.y + random()*aa - aa*0.5,
        )
        aa = 0.00000005
        self.orientation = Vector(random(),random())
        self.previous_orientation = Vector( 
            self.orientation.x + random()*aa - aa*0.5,
            self.orientation.y + random()*aa - aa*0.5,
        )


def target_only_movement_action(
    forward_max_speed: f32,
    max_speed_to_target: f32,
    slow_down_max_speed_to_target_ratio: f32,
    forward_max_angle: f32,
    max_rotation_speed: f32,
    slow_down_max_angle: f32,
    ship: Ship,
    target: Target,
) -> MovementAction:
    ship_v = wrap_around(ship.pp, ship.p).d
    ship_cross = normalize_2(normalize_2(ship.orientation) * 1.0 + normalize_2(ship_v) * 0.5)
    rotation_speed = cross(ship.orientation, ship.previous_orientation)
    speed = math.sqrt(wrap_around(ship.pp, ship.p).d_sqrd)
    wa1 = wrap_around(ship.pp, target.pp)
    wa2 = wrap_around(ship.p, target.p)
    distance_to_target = math.sqrt(wa2.d_sqrd)
    target_vs_ship_delta_v = math.sqrt(wa1.d_sqrd) - math.sqrt(wa2.d_sqrd)
    orientation_angle_corrected_2 = angle(normalize_2(ship_cross), normalize_2(target.p))
    orientation_angle_corrected = cross(normalize_2(ship_cross), normalize_2(target.p))
    max_aa = max(max_speed_to_target * 0.75, distance_to_target * slow_down_max_speed_to_target_ratio)
    if abs(orientation_angle_corrected_2) < slow_down_max_angle * 0.5 and target_vs_ship_delta_v > max_aa:
        return "slowdown"
    elif orientation_angle_corrected > 0.0 and rotation_speed > -max_rotation_speed:
        return "turnleft"
    elif orientation_angle_corrected < 0.0 and rotation_speed < max_rotation_speed:
        return "turnright"
    elif speed < forward_max_speed and abs(orientation_angle_corrected_2) < forward_max_angle * 0.5:
        return "forward"
    else:
        return "nothing"


# class Particle():
#     # ship.cross #
#     # ship.td 
#     aa = 0.001
#     def __init__(self):
#         self.p = Vector(random(),random())
#         self.pp = Vector( 
#             self.p.x + random()*aa - aa*0.5,
#             self.p.y + random()*aa - aa*0.5,
#         )


# class DataMaker():
#     def get_data(self):
#         ship = Vector(random(),random())
#         target = Vector(random(),random())
#         wa = wrap_around(ship, target)
#         decision = ""
#         if wa.d.y < 0 :
#             decision += "down"
#         if wa.d.y > 0 :
#             decision += "up"
#         if wa.d.x < 0 :
#             decision += "left"
#         if wa.d.x > 0 :
#             decision += "right"
#         return {
#             'sx': ship.x,
#             'sy': ship.y,
#             'tx': target.x,
#             'ty': target.y,
#             'decision': decision
#         }
# dm = DataMaker()


def get_data():
    ship = Ship()
    target = Target()
    decision = target_only_movement_action(
        0.0001, # forward_max_speed
        0.00001, # max_speed_at_target
        0.00025, # slow_down_max_speed_to_target_ratio
        30, # forward_max_angle
        0.000000004, # max_rotation_speed
        35,  # slow_down_max_angle
        ship,
        target,
    )
    decision_multi = {
        'slowdown': {
            'l': 0,
            'r': 0,
            'f': 1,
        },
        'turnleft': {
            'l': 0,
            'r': 1,
            'f': 0,
        },
        'turnright': {
            'l': 1,
            'r': 0,
            'f': 0,
        },
        'forward': {
            'l': 1,
            'r': 1,
            'f': 0,
        },
        'nothing': {
            'l': 0,
            'r': 0,
            'f': 0,
        }
    }[decision]
    return {
        'spx': ship.p.x,
        'spy': ship.p.y,
        'sppx': ship.pp.x,
        'sppy': ship.pp.y,
        'sox': ship.orientation.x,
        'soy': ship.orientation.y,
        'spox': ship.previous_orientation.x,
        'spoy': ship.previous_orientation.y,
        'tpx': target.p.x,
        'tpy': target.p.y,
        'tppx': target.pp.x,
        'tppy': target.pp.y,
        'decision': decision,
        'l':decision_multi['l'],
        'r':decision_multi['r'],
        'f':decision_multi['f'],
    }


columns = list(get_data().keys())
count = 256
df = pandas.DataFrame(
    [ get_data() for x in range(count) ]
)[columns]
logging.info(df[['decision']].value_counts())
df.to_csv("ai/train.csv", index=False)
(pandas.DataFrame(
    [ get_data() for x in range(int(count/4)) ]
)[columns]).to_csv("ai/test.csv", index=False)


df = pandas.read_csv("ai/train.csv")
r = df.iloc[0]
r = r.drop('decision')
logging.info(r.values)
