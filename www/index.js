import { memory } from "webgeo-hull/webgeo_hull_bg";

import * as wasm from "webgeo-hull";

// Configuration
const STEP_DELAY = 250;
const HEIGHT = 1000;
const WIDTH = 1000;
const POINT_SIZE = 5;
const LINE_WIDTH = 2;

const BLACK = "#000000"
const RED = "#FF0000";
const GREEN = "#00FF00";
const BLUE = "#0000FF";

const POINT_COLOR = RED;
const SEG_COLOR = "#000000";
const RAY_COLOR = "#FF0000";


// Set up wasm
var core = wasm.Core.new();

// Create control button
const control = document.getElementById("control-button");
control.addEventListener("click", event => {
    // Send signals to wasm
    switch (core.state()) {
        case 0: // Waiting
            core.start();
            control.innerText = "Step";
        break;
        case 1: // Running
            core.step();
            draw();
            if (core.state() == 2) {
                control.innerText = "Restart";
                control.disabled = false;
                skip.disabled = true;
                auto.disabled = true;
            }
        break;
        case 2: // Done
            core = wasm.Core.new();
            draw();
            control.innerText = "Start";
            skip.disabled = false;
            auto.disabled = false;
        break;
    }
});

// Create skip button
const skip = document.getElementById("skip-button");
skip.addEventListener("click", event => {
    if (core.state() == 0) {
        core.start();
    }
    if (core.state() != 2) {
        core.complete();
        draw();
        control.innerText = "Restart";
        control.disabled = false;
        skip.disabled = true;
        auto.disabled = true;
    }
});

// Create auto button
const auto = document.getElementById("auto-button");
auto.addEventListener("click", event => {
    if (core.state() == 0) {
        core.start();
    }
    if (core.state() != 2) {
        (function loop () {
            setTimeout(function () {
                core.step();
                draw();
                if (core.state() == 1) {
                    loop();
                }
                else {
                    control.disabled = false;
                    control.innerText = "Restart";
                }
            }, STEP_DELAY)
        })();
        control.disabled = true;
        auto.disabled = true;
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
        var len = core.input_length();
        var x = new Int32Array(memory.buffer, core.input_points_x(), len);
        var y = new Int32Array(memory.buffer, core.input_points_y(), len);
        for (let i = 0; i < len; i++) {
            ctx.beginPath();

            ctx.arc(x[i], y[i], POINT_SIZE, 0, 2 * Math.PI);

            ctx.fillStyle = POINT_COLOR;
            ctx.fill();
        }
    }

    // Points
    {
        var len = core.points_length();
        var x = new Int32Array(memory.buffer, core.points_x(), len);
        var y = new Int32Array(memory.buffer, core.points_y(), len);
        for (let i = 0; i < len; i++) {
            ctx.beginPath();

            ctx.arc(x[i], y[i], POINT_SIZE, 0, 2 * Math.PI);

            ctx.fillStyle = POINT_COLOR;
            ctx.fill();
        }
    }

    // Segments
    {
        var len = core.segs_length();
        var x = new Int32Array(memory.buffer, core.segs_x(), len * 2);
        var y = new Int32Array(memory.buffer, core.segs_y(), len * 2);
        for (let i = 0; i < len; i++) {
            ctx.beginPath();

            ctx.moveTo(x[i * 2], y[i * 2]);
            ctx.lineTo(x[(i * 2) + 1], y[(i * 2) + 1]);

            ctx.lineWidth = LINE_WIDTH;
            ctx.strokeStyle = SEG_COLOR;
            ctx.stroke();
        }
    }

    // Rays
    {
        var len = core.rays_length();
        var x = new Int32Array(memory.buffer, core.rays_x(), len * 2);
        var y = new Int32Array(memory.buffer, core.rays_y(), len * 2);
        for (let i = 0; i < len; i++) {
            ctx.beginPath();

            let out = extend(0, 0, WIDTH, HEIGHT, x[i * 2], y[i * 2], x[(i * 2) + 1], y[(i * 2) + 1]);
            ctx.moveTo(x[i * 2], y[i * 2]);
            ctx.lineTo(out[0], out[1]);

            ctx.lineWidth = LINE_WIDTH;
            ctx.strokeStyle = RAY_COLOR;
            ctx.stroke();
        }
    }

    // Other

    // Outline
    ctx.beginPath();
    ctx.strokeStyle = BLACK;
    ctx.rect(1, 1, WIDTH - 2, HEIGHT - 2);
    ctx.stroke();
}

// Add event listener for click input
canvas.addEventListener("click", event => {
    if (core.state() == 0) {
        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const x = (event.clientX - boundingRect.left) * scaleX;
        const y = (event.clientY - boundingRect.top) * scaleY;

        // Send signals to internal wasm
        core.input_add_point(x, y);
        draw();
    }
});

// Helper to extend rays
const extend = (xmin, ymin, xmax, ymax, x1, y1, x2, y2) => {
    if (x2 == x1) {
        if (y2 > y1) {
            return [x2, ymax];
        }
        else {
            return [x1, ymin];
        }
    }

    let slope = (y2 - y1) / (x2 - x1);
    let b = y1 - slope * x1;

    let left_y = slope * xmin + b;
    let right_y = slope * xmax + b;
    let bottom_x = (ymin - b) / slope;
    let top_x = (ymax - b) / slope;

    if ((ymin <= left_y) && (left_y <= ymax) && (x2 < x1)) {
        return [xmin, left_y];
    }

    if ((ymin <= right_y) && (right_y <= ymax) && (x2 > x1)) {
        return [xmax, right_y];
    }

    if ((xmin <= bottom_x) && (bottom_x <= xmax) && (y2 < y1)) {
        return [bottom_x, ymin];
    }

    if ((xmin <= top_x) && (top_x <= xmax) && (y2 > y1)) {
        return [top_x, ymax];
    }
}

// Draw board
draw();
