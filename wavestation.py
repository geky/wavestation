#!/usr/bin/env python3

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

class Xorshift64:
    def __init__(self, seed=None):
        if seed is None:
            seed = random.randint(1, 0xffffffffffffffff)
        self.seed = seed or 1
        self.state = self.seed

    def next(self):
        x = self.state
        x ^= (x << 13) & 0xffffffffffffffff
        x ^= (x >>  7) & 0xffffffffffffffff
        x ^= (x << 17) & 0xffffffffffffffff
        self.state = x
        return x/0x10000000000000000

    def range(self, start, stop=None, step=1):
        if stop is None:
            start, stop = 0, start
        size = (stop-start)//step
        i = math.floor(self.next() * size)
        return (i*step) + start

    def bool(self, p):
        return self.next() < p

    def poisson(self, p):
        count = 0
        while self.bool(p):
            count += 1
        return count

    def __iter__(self):
        return self.next()


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

def distsq(a, b):
    return (a[0]-b[0])**2 + (a[1]-b[1])**2
    
def dist(a, b):
    return math.sqrt(distsq(a, b))


def main(size, *,
        width=80,
        height=40,
        seed=None,
        bubble_p=0.5,
        extend_p=0.5,
        small_map=False,
        small_width=8,
        small_height=8,
        bubble_map=False):

    # initialize with either provided seed or actually random seed
    prng = Xorshift64(seed)
    print('seed: 0x%016x' % prng.seed)

    # initialize our first bubble with a random size
    bubbles = [Bubble(0, 0, 1+prng.poisson(bubble_p), None)]
    used = bubbles[0].r

    while used < size:
        # choose a bubble
        parent = bubbles[prng.range(len(bubbles))]
        # choose a direction
        dir = prng.range(4)
        # choose a size
        r = 1+prng.poisson(bubble_p)

        # calculate new position
        x = parent.x+dirx(dir)*(parent.r+r+1+prng.poisson(extend_p))
        y = parent.y+diry(dir)*(parent.r+r+1+prng.poisson(extend_p))

        # but wait, is there a collision?
        collision = False
        for bubble in bubbles:
            if bubble is parent:
                continue
            # check bubble collision
            if distsq((x,y), (bubble.x, bubble.y)) <= (r+bubble.r)**2:
                collision = True
                break
        if collision:
            continue

        # no? ok add to our bubbles
        bubbles.append(Bubble(x, y, r, parent))
        used += bubbles[-1].r

    # find bounds
    lower, upper = (0, 0), (1, 1)
    for bubble in bubbles:
        lower = (min(lower[0], bubble.x-bubble.r),
                min(lower[1], bubble.y-bubble.r))
        upper = (max(upper[0], bubble.x+bubble.r),
                max(upper[1], bubble.y+bubble.r))
    orig = lower
    width = upper[0]-lower[0]
    height = upper[1]-lower[1]
    print('widthxheight: %dx%d' % (width, height))

    # shift all bubbles into range 0,0 -> width,height
    for bubble in bubbles:
        bubble.x -= orig[0]
        bubble.y -= orig[1]

    # show small map?
    if small_map:
        scale = (small_width/width, small_height/height)
        smap = {(x,y): ' ' for x, y in it.product(
                range(small_width), range(small_height))}
        for bubble in bubbles:
            if bubble.parent is None:
                continue
            if bubble.x != bubble.parent.x:
                for x in range(
                        min(math.floor(bubble.x*scale[0]),
                            math.floor(bubble.parent.x*scale[0])),
                        max(math.floor(bubble.x*scale[0]),
                            math.floor(bubble.parent.x*scale[0]))):
                    if smap.get((x, math.floor(bubble.y*scale[1]))) == '|':
                        smap[(x, math.floor(bubble.y*scale[1]))] = '+'
                    else:
                        smap[(x, math.floor(bubble.y*scale[1]))] = '-'
            if bubble.y != bubble.parent.y:
                for y in range(
                        min(math.floor(bubble.y*scale[1]),
                            math.floor(bubble.parent.y*scale[1])),
                        max(math.floor(bubble.y*scale[1]),
                            math.floor(bubble.parent.y*scale[1]))):
                    if smap.get((math.floor(bubble.x*scale[0]), y)) == '-':
                        smap[(math.floor(bubble.x*scale[0]), y)] = '+'
                    else:
                        smap[(math.floor(bubble.x*scale[0]), y)] = '|'
        for bubble in bubbles:
            smap[(
                math.floor((bubble.x)*scale[0]),
                math.floor((bubble.y)*scale[1]))] = 'o'

        for y in range(small_height):
            print(''.join(smap[(x,y)] for x in range(small_width)))

    # render bubble map
    bmap = {(x,y): ' ' for x, y in it.product(range(width), range(height))}
    for bubble in bubbles:
        for x, y in it.product(range(width), range(height)):
            if dist((x,y), (bubble.x,bubble.y)) <= bubble.r:
                bmap[(x,y)] = '.'
    for bubble in bubbles:
        if bubble.parent is None:
            continue
        if bubble.x != bubble.parent.x:
            for x in range(
                    min(bubble.x, bubble.parent.x),
                    max(bubble.x, bubble.parent.x)):
                if bmap.get((x, bubble.y)) == '|':
                    bmap[(x, bubble.y)] = '+'
                else:
                    bmap[(x, bubble.y)] = '-'
        if bubble.y != bubble.parent.y:
            for y in range(
                    min(bubble.y, bubble.parent.y),
                    max(bubble.y, bubble.parent.y)):
                if bmap.get((bubble.x, y)) == '-':
                    bmap[(bubble.x, y)] = '+'
                else:
                    bmap[(bubble.x, y)] = '|'
    for bubble in bubbles:
        bmap[(bubble.x, bubble.y)] = 'o'

    # show bubble map?
    if bubble_map:
        for y in range(height):
            print(''.join(bmap[(x,y)] for x in range(width)))


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
        '--seed',
        type=lambda x: int(x, 0),
        default=None,
        help="Seed.")
    parser.add_argument(
        '-W', '--width',
        type=lambda x: int(x, 0),
        help="Render width.")
    parser.add_argument(
        '-H', '--height',
        type=lambda x: int(x, 0),
        help="Render height.")
    parser.add_argument(
        '--bubble-p',
        type=float,
        help="Probability to expand a bubble.")
    parser.add_argument(
        '--extend-p',
        type=float,
        help="Probability to extend a hallway.")
    parser.add_argument(
        '-s', '--small-map', '--small',
        action='store_true',
        help="Show small map.")
    parser.add_argument(
        '--small-width',
        type=lambda x: int(x, 0),
        help="Width of small map.")
    parser.add_argument(
        '--small-height',
        type=lambda x: int(x, 0),
        help="Height of small map.")
    parser.add_argument(
        '-b', '--bubble-map', '--bubble',
        action='store_true',
        help="Show bubble map.")
        
    sys.exit(main(**{k: v
        for k, v in vars(parser.parse_args()).items()
        if v is not None}))
