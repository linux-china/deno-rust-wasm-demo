import {assertEquals} from "https://deno.land/std@0.64.0/testing/asserts.ts";
import {add} from "./mod.ts";

Deno.test("add", () => {
        assertEquals(add(1, 1), 2);
    }
);
