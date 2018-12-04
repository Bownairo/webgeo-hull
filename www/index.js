import { memory } from "webgeo-hull/webgeo_hull_bg";

import * as wasm from "webgeo-hull";

// Configuration
const HEIGHT = 1000;
const WIDTH = 1000;
const POINT_SIZE = 5;
const LINE_WIDTH = 1;

const BLACK = "#000000"
const RED = "#FF0000";
const GREEN = "#00FF00";
const BLUE = "#0000FF";

const POINT_COLOR = RED;
const LINE_COLOR = "#000000";


// Set up wasm
const core = wasm.Core.new();

// Create control button
const control = document.getElementById("control-button");
const halt = new Uint8Array(memory.buffer, core.halt(), 1);
control.addEventListener("click", event => {
    // Send signals to wasm
    switch (core.state()) {
        case 0: // Waiting
            core.start();
            // XXX Change button text
        break;
        case 1: // Running
            // If halted, continue
            if (halt[0] == 1) {
                halt[0] = 1;
            }
        break;
        case 2: // Done

        break;
    }
});

// Setup canvas
const canvas = document.getElementById("webgeo-canvas");
canvas.height = HEIGHT;
canvas.width = WIDTH;

// Draw method
const draw = () => {
    const ctx = canvas.getContext('2d');
    ctx.clearRect(1, 1, WIDTH - 2,HEIGHT - 2);

    // Display state of wasm
    // Inputs
    if (core.state() == 0) {
        const len = core.input_length();
        const x = new Int32Array(memory.buffer, core.input_points_x(), len);
        const y = new Int32Array(memory.buffer, core.input_points_y(), len);
        for (let i = 0; i < len; i++) {
            ctx.beginPath();

            ctx.arc(x[i], y[i], POINT_SIZE, 0, 2 * Math.PI);

            ctx.fillStyle = POINT_COLOR;
            ctx.fill();
        }
    }

    // XXX Conditionally draw each of these
    // Points
    {
        const len = core.points_length();
        const x = new Int32Array(memory.buffer, core.points_x(), len);
        const y = new Int32Array(memory.buffer, core.points_y(), len);
        for (let i = 0; i < len; i++) {
            ctx.beginPath();

            ctx.arc(x[i], y[i], POINT_SIZE, 0, 2 * Math.PI);

            ctx.fillStyle = POINT_COLOR;
            ctx.fill();
        }
    }

    // Segments
    {
        const len = core.segs_length();
        const x = new Int32Array(memory.buffer, core.segs_x(), len * 2);
        const y = new Int32Array(memory.buffer, core.segs_y(), len * 2);
        for (let i = 0; i < len; i += 2) {
            ctx.beginPath();

            ctx.moveTo(x[i], y[i]);
            ctx.lineTo(x[i + 1], y[i + 1]);

            ctx.lineWidth = LINE_WIDTH;
            ctx.strokeStyle = LINE_COLOR;
            ctx.stroke();
        }
    }

    // Rays
    {
        const len = core.rays_length();
        const x = new Int32Array(memory.buffer, core.rays_x(), len * 2);
        const y = new Int32Array(memory.buffer, core.rays_y(), len * 2);
        for (let i = 0; i < len; i += 2) {
            ctx.beginPath();

            ctx.moveTo(x[i], y[i]);
            // XXX Where boundary and line intersect towards this point
            ctx.lineTo(x[i + 1], y[i + 1]);

            ctx.lineWidth = LINE_WIDTH;
            ctx.strokeStyle = LINE_COLOR;
            ctx.stroke();
        }
    }

    // Other

    // Outline
    ctx.beginPath();
    ctx.fillStyle = BLACK;
    ctx.rect(1, 1, WIDTH - 2, HEIGHT - 2);
    ctx.stroke();

    requestAnimationFrame(renderLoop);
}

// Canvas render loop
const renderLoop = () => {
    draw();
};

// Add event listener for click input
canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const x = (event.clientX - boundingRect.left) * scaleX;
    const y = (event.clientY - boundingRect.top) * scaleY;

    // Send signals to internal wasm
    core.input_add_point(x, y);
    draw();
});

// Kickoff animation
requestAnimationFrame(renderLoop);
