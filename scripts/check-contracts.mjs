import { z } from 'zod';
import { readFileSync, writeFileSync } from 'node:fs';
import { planContextSchema, planExchangeSchema } from '../src/kernel/plan/exchange.ts';
import { manifestSchema } from '../src/kernel/modules/model.ts';
for (const [name, schema] of [
  ['plan-context', planContextSchema],
  ['plan-exchange', planExchangeSchema],
  ['module-manifest', manifestSchema],
]) {
  const path = `docs/contracts/${name}.schema.json`,
    expected = z.toJSONSchema(schema);
  if (process.argv.includes('--write'))
    writeFileSync(path, JSON.stringify(expected, null, 2) + '\n');
  else if (JSON.stringify(JSON.parse(readFileSync(path, 'utf8'))) !== JSON.stringify(expected))
    throw Error(`Contract drift: ${path}`);
}
console.log(
  'Module / Plan Context / Plan Exchange JSON schema matches source validators. Cross-field proposal checks remain in TS/Rust validators.',
);
