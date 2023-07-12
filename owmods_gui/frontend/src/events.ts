import { listen as tauriListen, emit as tauriEmit } from "@tauri-apps/api/event";
import { Event } from "@types";

type Params<E extends Event["name"]> = Extract<Event, { name: E }>["params"];

type EventSubscriptions = {
    [E in Event["name"]]: Array<(params: Params<E>) => void>;
};

let initialized = false;
const subscriptions = {} as EventSubscriptions;

export const listen = async <E extends Event["name"]>(
    name: E,
    callback: (params: Params<E>) => void
) => {
    console.debug(initialized);
    if (!initialized) {
        initialized = true;
        await tauriListen("owmods://events/invoke", (e) => {
            const payload = e.payload as Event;
            if (subscriptions[payload.name]) {
                for (const handler of subscriptions[payload.name]) {
                    (handler as (params: Params<typeof payload.name>) => void)(payload.params);
                }
            }
        });
    }
    if (subscriptions[name] === undefined) {
        subscriptions[name] = [];
    }
    subscriptions[name].push(callback);
};

export const emit = async <E extends Event["name"]>(name: E, params: Params<E>) => {
    return tauriEmit("owmods://events/invoke", {
        name,
        params
    } as Event);
};
