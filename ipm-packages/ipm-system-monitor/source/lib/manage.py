import subprocess
from typing import List, NamedTuple
import sys, os
from network import *

class TasksInfo(NamedTuple):
    total: int
    running: int
    sleeping: int
    stopped: int
    zombie: int

class TaskInfo(NamedTuple):
    pid: int
    user: str
    pr: int
    ni: int
    virt: float
    res: float
    shr: int
    s: str
    cpu: float
    mem: float
    time: str
    command: str
    
class CpuInfo(NamedTuple):
    user_usage: float
    system_usage: float
    nice_usage: float
    idle: float
    IO_wait_usage: float
    hardware_interrupt_usage: float
    software_interrupt_usage: float
    stolen_usage: float

class MemInfo(NamedTuple):
    total: float
    free: float
    used: float
    buff_cache: float

class SwapInfo(NamedTuple):
    total: float
    free: float
    used: float
    avail_mem: float

class MachineInfo(NamedTuple):
    task: TasksInfo
    cpu: CpuInfo
    memory: MemInfo
    swap: SwapInfo
    processes_txt: List[str]

def cmd(command: str) -> str:
    result = ""
    try:
        result = subprocess.check_output(command, shell=True, text=True)
    except subprocess.CalledProcessError as e:
        print(f"Failed to execute command: {command}", file=sys.stderr)
    return result

def string_split(s: str, delimiter: str) -> List[str]:
    return s.split(delimiter)

def string_between(s: str, start: str, end: str) -> str:
    return s.split(start)[1].split(end)[0]

def info_txt() -> List[str]:
    return string_split(cmd("top -b -n 1 -w 512"), '\n')

def get_tasks_data(s: str) -> TasksInfo:
    return TasksInfo(
        int(string_between(s, "Tasks: ", " total, ")),
        int(string_between(s, " total, ", " running, ")),
        int(string_between(s, " running, ", " sleeping, ")),
        int(string_between(s, " sleeping, ", " stopped, ")),
        int(string_between(s, " stopped, ", " zombie"))
    )
def get_task_data(s: str) -> TaskInfo:
    data = [e for e in s.split(" ") if e != ""]
    if data[4].endswith("g"):
        data[4] = int(data[4][:-1] * 1024 ** 3)
    else:
        data[4] = int(data[4])
    if data[5].endswith("g"):
        data[5] = int(data[5][:-1] * 1024 ** 3)
    else:
        data[5] = int(data[5])
    
    return TaskInfo(
        int(data[0]),
        data[1],
        int(data[2]),
        int(data[3]),
        data[4],
        data[5],
        int(data[6]),
        data[7],
        float(data[8]),
        float(data[9]),
        data[10],
        " ".join(data[11:])
    )


def get_cpu_data(s: str) -> CpuInfo:
    kws = [":", "us,", "sy,", "ni,", "id,", "wa,", "hi,", "si,", "st"]
    return CpuInfo(
        float(string_between(s, kws[0], kws[1])),
        float(string_between(s, kws[1], kws[2])),
        float(string_between(s, kws[2], kws[3])),
        float(string_between(s, kws[3], kws[4])),
        float(string_between(s, kws[4], kws[5])),
        float(string_between(s, kws[5], kws[6])),
        float(string_between(s, kws[6], kws[7])),
        float(string_between(s, kws[7], kws[8]))
    )
def get_cpu_usage() -> float:
    return 100 - get_cpu_data(info_txt()[2]).idle

def get_cpu_cores() -> int:
    return int(cmd("grep processor /proc/cpuinfo | wc -l"))
CPU_CORES_COUNT = get_cpu_cores()

def get_cpu_core_usage(core_index) -> float:
    if core_index >= CPU_CORES_COUNT or core_index < 0:
        return -1
    def info_txt_cores():
        gotten_data=cmd(f"top -1 -b -n 1 -w 512").replace("st   %Cpu","st\n%CPU")
        print(gotten_data)
        return string_split(gotten_data, '\n')
    return 100 - get_cpu_data(info_txt_cores()[2 + core_index]).idle
def get_cpu_core_usages() -> List[float]:
    gotten_data=cmd(f"top -1 -b -n 1 -w 512").replace("st   %Cpu","st\n%CPU")
    gotten_data_list=string_split(gotten_data, '\n')
    result = []
    for i in range(CPU_CORES_COUNT):
        result.append(100 - get_cpu_data(gotten_data_list[2+i]).idle)
    return result
    
def get_mem_data(s: str) -> MemInfo:
    kws = ["Mem : ", " total, ", " free, ", " used, ", " buff/cache"]
    return MemInfo(
        float(string_between(s, kws[0], kws[1])),
        float(string_between(s, kws[1], kws[2])),
        float(string_between(s, kws[2], kws[3])),
        float(string_between(s, kws[3], kws[4]))
    )

def get_swap_data(s: str) -> SwapInfo:
    kws = ["Swap: ", " total, ", " free, ", " used. ", " avail Mem"]
    return SwapInfo(
        float(string_between(s, kws[0], kws[1])),
        float(string_between(s, kws[1], kws[2])),
        float(string_between(s, kws[2], kws[3])),
        float(string_between(s, kws[3], kws[4]))
    )
def get_mem_usage() -> float:
    get_data=get_mem_data(info_txt()[3])
    return 100 - (get_data.used + get_data.buff_cache) / get_data.total * 100
def info() -> MachineInfo:
    info_txt_data = info_txt()
    task_info_data = get_tasks_data(info_txt_data[1])
    cpu_info_data = get_cpu_data(info_txt_data[2])
    mem_info_data = get_mem_data(info_txt_data[3])
    swap_info_data = get_swap_data(info_txt_data[4])
    processes = info_txt_data[7:]
    return MachineInfo(task_info_data, cpu_info_data, mem_info_data, swap_info_data, processes)
