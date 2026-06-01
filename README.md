# griot-math-wasm

> Living memory mathematics compiled to WebAssembly — griot oral tradition as exponential decay and federated memory.

## What This Does

`griot-math-wasm` is the WebAssembly build of griot mathematics. Stories with exponential decay, genealogy trees, praise names, call-and-response, and federation — all running at near-native speed in the browser. Use it for web-based knowledge management, browser caching with cultural decay, or educational tools about oral traditions.

## The Cultural Root

See `griot-math` (PyPI) for the full cultural background. West African griots maintain oral histories where frequently told stories resist decay.

## Install

```bash
npm install griot-math-wasm
```

## Quick Start

```typescript
import init, { Griot, generate_praise, call_and_response, Federation } from "griot-math-wasm";

await init();

const griot = new Griot(0.01);
const s1 = griot.add_story("The founding", 1.0, null, ["history"]);
const s2 = griot.add_story("The flood", 0.8, s1, ["disaster"]);

griot.tell_story("The founding");
griot.apply_decay(3600);

const strengths = griot.memory_strengths();
const score = griot.tradition_score();

// Praise names
const praise = generate_praise(griot, [s1, s2], "Keeper");

// Federation
const g2 = new Griot(0.01);
const fed = new Federation([griot, g2]);
fed.sync_story(0, 1, s2);
```

## API Reference

### `Griot(decayRate?)`
- `add_story(name, weight, parentId?, tags?) → string`
- `tell_story(name) → number`
- `apply_decay(elapsedMs) → void`
- `memory_strengths() → Float64Array`
- `tradition_score() → number`

### `generate_praise(griot, storyIds, name) → PraiseName`
### `call_and_response(caller, responder, callerStoryId) → CallResponse`
### `genealogy(griot, storyId) → Uint32Array`
### `Federation(griots)` — `sync_story()`, `merge_memories()`, `coverage()`

## License

MIT
