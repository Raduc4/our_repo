import test from "ava";

import { predict } from "../index.js";

test("sum from native", (t) => {
  t.is(predict([0, 0, 0, 5, 6, 3]), 1.1);
});
