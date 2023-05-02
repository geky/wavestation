#!/usr/bin/env python3

# TODO allow seed argument
# TODO just use xorshift?

import random
import itertools as it
import math

class Bubble:
    def __init__(self, x, y, r, parent):
        self.x = x
        self.y = y
        self.r = r
        self.parent = parent

    def __repr__(self):
        return repr((self.x, self.y, self.r))

def main(size, width, height, expand_prob, expand_off):
    def randsize():
        bsize = 1
        while True:
            if not (random.random() < expand_prob):
                break
            bsize +=1
        return bsize

    def dirx(dir):
        if dir == 1:
            return +1
        elif dir == 3:
            return -1
        else:
            return 0

    def diry(dir):
        if dir == 0:
            return +1
        elif dir == 2:
            return -1
        else:
            return 0

    def dist(a, b):
        xd = a[0]-b[0]
        yd = a[1]-b[1]
        return math.sqrt(xd*xd + yd*yd)

    # initialize our first bubble with a random size
    bubbles = [Bubble(0,0,randsize(),None)]
    used = bubbles[0].r

    while used < size:
        # choose a bubble
        parent = bubbles[random.randrange(len(bubbles))]
        # choose a direction
        dir = random.randrange(4)
        # choose a size
        r = randsize()

        # calculate new position
        x = parent.x+dirx(dir)*(parent.r+r+expand_off)
        y = parent.y+diry(dir)*(parent.r+r+expand_off)

        # but wait, is there a collision?
        collision = False
        for bubble in bubbles:
            if dist((x,y), (bubble.x, bubble.y)) <= r+bubble.r:
                collision = True
                break
        if collision:
            continue

        # no? ok add to our bubbles
        bubbles.append(Bubble(x, y, r, parent))
        used += bubbles[-1].r

    map = {(x,y): ' ' for x, y in it.product(range(width), range(height))}
    for bubble in bubbles:
        for x, y in it.product(range(width), range(height)):
            if dist((x,y), (bubble.x+width//2,bubble.y+height//2)) <= bubble.r:
                map[(x,y)] = '.'
    for bubble in bubbles:
        if bubble.parent is None:
            continue
        if bubble.x != bubble.parent.x:
            for x in range(
                    min(bubble.x, bubble.parent.x),
                    max(bubble.x, bubble.parent.x)):
                map[(x+width//2, bubble.y+height//2)] = '-'
        if bubble.y != bubble.parent.y:
            for y in range(
                    min(bubble.y, bubble.parent.y),
                    max(bubble.y, bubble.parent.y)):
                map[(bubble.x+width//2, y+height//2)] = '|'
    for bubble in bubbles:
        map[(bubble.x+width//2, bubble.y+height//2)] = 'o'

    for y in range(height):
        print(''.join(map[(x,y)] for x in range(width)))


if __name__ == "__main__":
    import sys
    import argparse
    parser = argparse.ArgumentParser(
        description="Generate!",
        allow_abbrev=False)
    parser.add_argument(
        'size',
        type=lambda x: int(x, 0),
        help="Size.")
    parser.add_argument(
        '-W', '--width',
        type=lambda x: int(x, 0),
        default=40,
        help="Render width.")
    parser.add_argument(
        '-H', '--height',
        type=lambda x: int(x, 0),
        default=20,
        help="Render height.")
    parser.add_argument(
        '--expand-prob',
        type=float,
        default=0.5,
        help="Probability to expand capsule.")
    parser.add_argument(
        '--expand-off',
        type=lambda x: int(x, 0),
        default=1,
        help="Distance from capsule new capsules are created.")

    sys.exit(main(**{k: v
        for k, v in vars(parser.parse_args()).items()
        if v is not None}))