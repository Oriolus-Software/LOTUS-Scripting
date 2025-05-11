import { Hono } from "hono";

const app = new Hono();

app.get("/", (c) => c.redirect("/lotus_script"));

export default app;
