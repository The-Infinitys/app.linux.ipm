from importer import *

def Color_HSV(h,s,v):
    h=h/360
    s=s/100
    v=v/100
    if s == 0.0: v*=255; return (v, v, v)
    i = int(h*6.) # XXX assume int() truncates!
    f = (h*6.)-i; p,q,t = int(255*(v*(1.-s))), int(255*(v*(1.-s*f))), int(255*(v*(1.-s*(1.-f)))); v*=255; i%=6
    p=int(p)
    q=int(q)
    v=int(v)
    t=int(t)
    if i == 0: return Color(v, t, p)
    if i == 1: return Color(q, v, p)
    if i == 2: return Color(p, v, t)
    if i == 3: return Color(p, q, v)
    if i == 4: return Color(t, p, v)
    if i == 5: return Color(v, p, q)

mem_usage_data = []
swap_usage_data = []
network_usage_data = []
# manage.get_network_usage()

class InfinitySystemMonitor(App):
    CSS = """
    Screen {
      align: center top;
    }
    #cpu-usage, #mem-usage, #network-usage {
      text-align: right;
    }
    #cpu-usage {
      color: red;
    }
    #mem-usage{
      color: yellow;
    }
    #network-usage{
      color: green;
    }
    Static.item-title {
      border: heavy white;
    }
    ProgressBar.core-usage {
      padding-left: 1;
    }
    #mem-swap-usage {
      padding-left: 1;
      padding-right: 1;
      overflow: hidden;
    }
    #network-graph {
      padding-left: 1;
      padding-right: 1;
      overflow: hidden;
    }
    """
    CSS+="""
    #cpu-core-usage {
      padding-left: 1;
      padding-right: 1;
      overflow: hidden;
      height: """+str(manage.CPU_CORES_COUNT + 1)+""";  
    }"""
    ENABLE_COMMAND_PALETTE=False
    BINDINGS = [
      ("q", "quit_app()", "Quit the application"),
      ]
    def compose(self) -> ComposeResult:
        # Compose the UI
        yield Header("The Infinity's System Monitor")
        # Create the CPU and memory usage widgets
        yield Static("CPU", classes="item-title")
        yield Static("CPU USAGE", id="cpu-usage")
        yield Canvas(shutil.get_terminal_size().columns - 4, 2 * manage.CPU_CORES_COUNT, Color(0, 0, 0),id="cpu-core-usage")
        yield Static("MEMORY and SWAP",classes="item-title")
        yield Static("SWAPORY USAGE", id="mem-usage")
        yield Canvas(shutil.get_terminal_size().columns - 4, 20, Color(0, 0, 0),id="mem-swap-usage")
        # Create Network Usage
        yield Static("NETWORK", classes="item-title")
        yield Static("NETWORK USAGE", id="network-usage")
        yield Canvas(shutil.get_terminal_size().columns - 4, 20, Color(0, 0, 0),id="network-graph")
        # Create the footer
        yield Footer()
    def update(self) -> None:
        # Get the machine info
        machine_info = manage.info()
        # Calculate the CPU and memory usage
        term_size = shutil.get_terminal_size()
        term_width = term_size.columns
        term_height = term_size.lines
        graph_height = int( 2 * (term_height
                        - 2 # the height of header and footer
                        - 4*3 # the height of item-title and data strings
                        - 2 # padding
                        - manage.CPU_CORES_COUNT * 2 # the height of cpu-core-usage
                        ) / 2)
        round_level = 0
        cpu_usage = int(10 ** round_level * (100 - machine_info.cpu.idle)) / 10 ** round_level
        mem_used = int(10 ** round_level * 100 * (machine_info.memory.used/machine_info.memory.total)) / 10 ** round_level
        mem_buffcache = int(10 ** round_level * 100 * (machine_info.memory.buff_cache/machine_info.memory.total)) / 10 ** round_level
        # Calculate the swap usage
        swap_used=0
        if machine_info.swap.total != 0:
          swap_used = int(10 ** round_level * 100 * (machine_info.swap.used/machine_info.swap.total)) / 10 ** round_level
        # Calculate the network usage
        network_usage = manage.get_network_usage()
        # Update the UI
        self.query_one("#cpu-usage").update(f"{cpu_usage}%")
        self.query_one("#mem-usage").update(f"{mem_used}% (buff/cache: {mem_buffcache}%)")
        self.query_one("#network-usage").update(f"RX: {manage.byte_human_readable(network_usage[0])}/s TX: {manage.byte_human_readable(network_usage[1])}/s")
        # Update the CPU core usage canvas
        core_usage = manage.get_cpu_core_usages()
        core_usage_canvas=self.query_one("#cpu-core-usage")
        core_usage_canvas._width = term_width - 4
        core_usage_canvas.clear()
        for i in range(manage.CPU_CORES_COUNT):
            cpu_core_usage = int(10 ** round_level * core_usage[i]) / 10 ** round_level
            resized_usage=int(core_usage_canvas._width * cpu_core_usage/100)
            core_usage_canvas.draw_line(0,i*2,resized_usage,i*2,Color_HSV(360 * i / manage.CPU_CORES_COUNT,100,100))
        # Update the memory usage canvas
        mem_usage_canvas=self.query_one("#mem-swap-usage")
        mem_usage_canvas._width = term_width - 4
        mem_usage_canvas._height = graph_height
        mem_usage_canvas.clear()
        # Update the memory usage data
        mem_usage_data.insert(0,[mem_used,mem_buffcache])
        swap_usage_data.insert(0,swap_used)
        # Remove the old data
        if len(mem_usage_data) > mem_usage_canvas._width:
            for i in range(len(mem_usage_data) - mem_usage_canvas._width):
                mem_usage_data.pop()
                swap_usage_data.pop()
        for i in range(min(mem_usage_canvas._width, len(mem_usage_data))):
            # Draw the memory usage data
            canv_width = mem_usage_canvas._width
            canv_height = mem_usage_canvas._height
            draw_x = canv_width - i
            draw_y_start = canv_height
            draw_y_end = int(canv_height * (100 - mem_usage_data[i][0]) / 100)
            try:
                mem_usage_canvas.draw_line(draw_x, draw_y_start, draw_x, draw_y_end, Color_HSV(60,100,100))
            except IndexError:
                pass
            draw_y_start = draw_y_end
            draw_y_end -= int(canv_height * (mem_usage_data[i][1]) / 100)
            try:
                mem_usage_canvas.draw_line(draw_x, draw_y_start, draw_x, draw_y_end, Color_HSV(60,50,100))
            except IndexError:
              pass
            if machine_info.swap.total != 0:
                draw_y_start = canv_height
                draw_y_end = int(canv_height * (100 - swap_usage_data[i]) / 100)
                try:
                    mem_usage_canvas.draw_line(draw_x, draw_y_start, draw_x, draw_y_end, Color_HSV(0,100,100))
                except IndexError:
                    pass
        # Update the network usage canvas
        network_usage_data.insert(0,network_usage)
        network_usage_canvas=self.query_one("#network-graph")
        network_usage_canvas._width = term_width - 4
        network_usage_canvas._height = graph_height
        network_usage_canvas.clear()
        # Remove the old data
        if len(network_usage_data) > network_usage_canvas._width:
            for i in range(len(network_usage_data) - network_usage_canvas._width):
                network_usage_data.pop()
        draw_y_start_received = 0
        draw_y_end_received = 0
        draw_y_start_transmitted = 0
        draw_y_end_transmitted = 0
        for i in range(int(min(network_usage_canvas._width / 5 + 1, len(network_usage_data)))):
            def check_received(data):
                return data[0]
            def check_transmitted(data):
                return data[1]
            # Draw the network usage data
            canv_width = network_usage_canvas._width
            canv_height = network_usage_canvas._height
            draw_x_start = canv_width - i * 5 + 5
            draw_x_end = draw_x_start - 5
            draw_y_start_received = draw_y_end_received
            draw_y_end_received = int(canv_height * (1 - network_usage_data[i][0] / max(network_usage_data, key=check_received)[0]))
            # Draw the received data
            try:
                network_usage_canvas.draw_line(draw_x_start, draw_y_start_received, draw_x_end, draw_y_end_received, Color_HSV(120,100,100))
            except IndexError:
                pass
            draw_y_start_transmitted = draw_y_end_transmitted
            draw_y_end_transmitted = int(canv_height * (1 - network_usage_data[i][1] / max(network_usage_data, key=check_transmitted)[1]))
            # Draw the transmitted data
            try:
                network_usage_canvas.draw_line(draw_x_start, draw_y_start_transmitted, draw_x_end, draw_y_end_transmitted, Color_HSV(120,50,100))
            except IndexError:
                pass
            
    def on_mount(self) -> None:
        # Initialize the theme
        infinite_theme = Theme(
          name="infinite",
          primary="#999999",
          secondary="#3333FF",
          accent="#11FFFF",
          foreground="#EEEEEE",
          background="#111111",
          success="#00FFFF",
          warning="#FFFF00",
          error="#FF0000",
          surface="#000000",
          panel="#333333",
          dark=True,
          variables={
              "block-cursor-text-style": "none",
              "footer-key-foreground": "#00ffff",
          },
        )
        # Register the theme
        self.register_theme(infinite_theme)
        self.theme = "infinite"
        self.set_interval(0.1, self.update)

    def action_quit_app(self) -> None:
        self.exit(0)
    def action_nothing(self) -> None:
        pass


# Run the application
if __name__ == "__main__":  
    app = InfinitySystemMonitor()
    app.run()
