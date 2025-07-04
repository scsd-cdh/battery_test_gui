<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
import { LineChart } from "@/components/ui/chart-line";
import { Badge } from "@/components/ui/badge";
import { useBatteryManager } from "@/lib/battery.ts";
import { invoke } from "@tauri-apps/api/tauri"; // added to use the export_to_csv() from backend and invoke tauri commands
import { open } from "@tauri-apps/api/dialog";
import { ref } from "vue";

interface TimeOption {
  label: string;
  value: number;
}

const timeOptions: TimeOption[] = [
  { label: "Last 5 sec", value: 5 },
  { label: "Last 30 sec", value: 30 },
  { label: "Last 1 min", value: 60 },
  { label: "Last 2 min", value: 120 },
  { label: "Last 5 min", value: 300 },
];

const selectedInterval = ref(5);

const batteryManager = useBatteryManager(selectedInterval);

const batteries = batteryManager.latest_data;
const open_ports = batteryManager.open_ports;
const voltages = batteryManager.batteries_voltages;
const currents = batteryManager.batteries_currents;
const battery_temp = batteryManager.batteries_temperatures;
const bench_temp = batteryManager.battery_benches_temperatures;
const elec_temp = batteryManager.bench_loads_temperatures;

// invokes the export_csv_command tauri command and creates the csv file in the project's main directory (supposed to)
async function exportToCSV() {
  try {
    const projectDir = await open({
      directory: true,
      multiple: false,
    });
    console.log("Project Directory:", projectDir); // Debug
    const csvPath = projectDir;
    await invoke("export_csv_command", { csvPath });
    alert("CSV export successful!");
  } catch (error) {
    console.error("Failed to export CSV:", error);
    alert("Failed to export CSV.");
  }
}
</script>

<template>
  <section class="m-10 flex flex-col gap-10">
    <!--Top Section-->
    <section>
      <h1 class="text-2xl font-bold">Batteries Connected</h1>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Battery Port</TableHead>
            <TableHead>Voltage</TableHead>
            <TableHead>Current</TableHead>
            <TableHead>Temperature</TableHead>
            <TableHead>Bench Temperature</TableHead>
            <TableHead>Electronic Load Temperature</TableHead>
            <TableHead>Duration</TableHead>
            <TableHead>Bench State</TableHead>
            <TableHead>Test Status</TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>

        <!--Table Battery-->
        <TableBody>
          <TableRow v-for="battery in batteries">
            <TableCell>{{ battery.port }}</TableCell>
            <TableCell>{{ battery.voltage / 100 }}V</TableCell>
            <TableCell>{{ battery.current / 100 }}mA</TableCell>
            <TableCell>{{ battery.battery_temperature / 100 }}C</TableCell>
            <TableCell>{{ battery.temperature / 100 }}C</TableCell>
            <TableCell
              >{{ battery.electronic_load_temperature / 100 }}C</TableCell
            >
            <TableCell
              >{{ +battery.end_date - +battery.start_date }}
            </TableCell>
            <TableCell>
              <Badge variant="secondary"> Standby </Badge>
            </TableCell>
            <TableCell>
              <Badge variant="secondary"> Standby </Badge>
            </TableCell>
            <TableCell class="text-right">
              <Button>Begin Test</Button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </section>

    <!-- Time Filter Dropdown -->
    <div class="mb-4 flex items-center gap-4">
      <label class="font-semibold">Trim history older than:</label>

      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button>
            {{
              timeOptions.find(
                (opt: TimeOption) => opt.value === selectedInterval,
              )?.label
            }}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem
            v-for="option in timeOptions"
            :key="option.value"
            @click="selectedInterval = option.value"
          >
            {{ option.label }}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>

    <section class="grid grid-cols-2 gap-5">
      <h2 class="text-2xl font-bold">Voltage [V]</h2>
      <h2 class="text-2xl font-bold">Current [mA]</h2>

      <!--Voltage chart-->
      <LineChart
        class="max-h-64"
        :data="voltages"
        index="index"
        :categories="open_ports"
      />

      <!--current chart-->
      <LineChart
        class="max-h-64"
        :data="currents"
        index="index"
        :categories="open_ports"
      />
    </section>

    <section class="grid grid-cols-3 gap-5">
      <h2 class="text-2xl font-bold">Battery Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Bench Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Electronic Load Temperature [C]</h2>

      <LineChart
        class="max-h-64"
        :data="battery_temp"
        index="index"
        :categories="open_ports"
      />
      <LineChart
        class="max-h-64"
        :data="bench_temp"
        index="index"
        :categories="open_ports"
      />
      <LineChart
        class="max-h-64"
        :data="elec_temp"
        index="index"
        :categories="open_ports"
      />
    </section>
    <section>
      <Button @click="exportToCSV">Export to CSV</Button>
    </section>
  </section>
</template>
