import time
from time import sleep


_new = [0, 0]
_after = time.time()
_old = [0, 0]
_before = time.time()

def get_network_usage() -> list[int]:
    global _new, _after, _old, _before
    datas = []
    _old = _new
    _before = _after
    with open("/proc/net/dev", mode="r") as f:
        for l in f.readlines():
            if ":" in l:
                data = [e for e in l.split(" ") if e != ""]
                datas.append(data)
    _after = time.time()
    _new = [0, 0]
    for data in datas:
        _new[0]+=int(data[1])
        _new[1]+=int(data[9])
    return [int((_new[i] - _old[i]) / (_after - _before)) for i in range(2)]

def byte_human_readable(b: int) -> str:
    byte_units=["B","KB","MB","GB","TB","PB","EB","ZB","YB"]
    i = 0
    while b >= 1024:
        b /= 1024
        i += 1
    return f"{b:.2f} {byte_units[i]}"

def bit_human_readable(b: int) -> str:
    bit_units=["b","Kb","Mb","Gb","Tb","Pb","Eb","Zb","Yb"]
    i = 0
    while b >= 1024:
        b /= 1024
        i += 1
    return f"{b:.2f} {bit_units[i]}"
