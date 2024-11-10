import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event';


interface Response {
  ok: boolean,
  error?: string,
}

async function register(id: string): Promise<Response> {
  return await invoke<Response>('plugin:polygon|register', {
    id,
  }).then((r: Response) => r);
}

async function registerAll(ids: [string]): Promise<Response> {
  return await invoke<Response>('plugin:polygon|register_all', {
    id: ids,
  }).then((r: Response) => r);
}

async function remove(id: string): Promise<Response> {
  return await invoke<Response>('plugin:polygon|remove', {
    id,
  }).then((r: Response) => r);
}

async function clear(): Promise<Response> {
  return await invoke<Response>('plugin:polygon|clear', {}).then((r: Response) => r);
}

async function show(id: string): Promise<Response> {
  return await invoke<Response>('plugin:polygon|show', {
    id,
  }).then((r: Response) => r);
}

async function hide(id: string): Promise<Response> {
  return await invoke<Response>('plugin:polygon|hide', {
    id,
  }).then((r: Response) => r);
}

async function update(id: string, points: [[number, number]]): Promise<Response> {
  return await invoke<Response>('plugin:polygon|update', {
    id,
    points
  }).then((r: Response) => r);
}


const POLYGON_LEFT_CLICK = "POLYGON_LEFT_CLICK";
const POLYGON_DOUBLE_CLICK = "POLYGON_DOUBLE_CLICK";
const POLYGON_RIGHT_CLICK = "POLYGON_RIGHT_CLICK";
const POLYGON_DRAG = "POLYGON_DRAG";
const POLYGON_MOUSE_MOVE = "POLYGON_MOUSE_MOVE";
const POLYGON_WHEEL = "POLYGON_WHEEL";
const POLYGON_ERROR = "POLYGON_ERROR";

type Payload = ClickPayload | DragPayload | ErrorPayload | WheelPayload;
type EventCallback = (payload: Payload) => void;

interface ClickPayload {
  position: { x: number, y: number },
}

interface WheelPayload {
  delta: { x: number, y: number },
}

interface DragPayload {
  from: {x: number, y: number},
  to: {x: number, y: number}
}

interface ErrorPayload {
  error: string
}
type Event = "LeftClick" | "DoubleClick" | "RightClick" | "Drag" | "MouseMove" | "Wheel" | "Error";
const Events = ["LeftClick", "DoubleClick", "RightClick", "Drag", "MouseMove", "Wheel", "Error"];

const EventCallbacks = {
  LeftClick: [] as EventCallback[],
  DoubleClick: [] as EventCallback[],
  RightClick: [] as EventCallback[],
  Drag: [] as EventCallback[],
  Wheel: [] as EventCallback[],
  MouseMove: [] as EventCallback[],
  Error: [] as EventCallback[],
}

function on(evt: Event, callback: (payload: Payload) => void) {
  if (!Events.includes(evt)) {
    throw new Error(`Event [${evt}] does not exist. Available event: ${Events.join(', ')}`);
  }
  EventCallbacks[evt].push(callback);
}

function off(evt: Event, callback: (payload: Payload) => void) {
  if (!Events.includes(evt)) {
    throw new Error(`Event [${evt}] does not exist. Available event: ${Events.join(', ')}`);
  }
  EventCallbacks[evt] = EventCallbacks[evt].filter((c) => c !== callback);
}

listen(POLYGON_LEFT_CLICK, async ev => {
  EventCallbacks.LeftClick.forEach(callback => callback(ev.payload as Payload));
})

listen(POLYGON_DOUBLE_CLICK, async ev => {
  EventCallbacks.DoubleClick.forEach(callback => callback(ev.payload as Payload));
})

listen(POLYGON_RIGHT_CLICK, async ev => {
  EventCallbacks.RightClick.forEach(callback => callback(ev.payload as Payload));
})

listen(POLYGON_DRAG, async ev => {
  EventCallbacks.Drag.forEach(callback => callback(ev.payload as Payload));
})

listen(POLYGON_WHEEL, async ev => {
  EventCallbacks.Wheel.forEach(callback => callback(ev.payload as Payload));
})

listen(POLYGON_MOUSE_MOVE, async ev => {
  EventCallbacks.MouseMove.forEach(callback => callback(ev.payload as Payload));
})

listen(POLYGON_ERROR, async ev => {
  EventCallbacks.Error.forEach(callback => callback(ev.payload as Payload));
})

export const polygon = {
  register,
  registerAll,
  remove,
  clear,
  show,
  hide,
  update,
  on,
  off
}