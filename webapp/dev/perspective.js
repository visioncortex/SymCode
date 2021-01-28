import { jsgl } from "./jsgl";

// By Thatcher Ulrich http://tulrich.com 2009
// Adapted for Symcode testing
//
// This source code has been donated to the Public Domain.  Do
// whatever you want with it.  Use at your own risk.

var images = [];
var canvas_elem = null;
var c3d = null;
var temp_mat0 = null;
var temp_mat1 = null;
var temp_mat2 = null;
var object_mat = null;
var camera_mat = null;
var proj_mat = null;
var options = {
  draw_backfaces: false,
  whiteout_alpha: 1,
  wireframe: false,
  subdivide_factor: 10.0,
  nonadaptive_depth: 7,
  draw_quad: true,
};
var horizontal_fov_radians = Math.PI / 2;
var target_distance = 1.9;
var draw_wireframe = false;

var MIN_Z = 0.05;

// Return two points, one slightly on either side of the midpoint. Use
// this to cover up seams between triangles.
function bisectFat(p0, p1, lenEstimate) {
    var f0 = 0.5 * (lenEstimate + 5) / lenEstimate;
    var f1 = 0.5 * (lenEstimate - 5) / lenEstimate;
    var ps = [
        {x: p0.x + (p1.x - p0.x) * f0,
         y: p0.y + (p1.y - p0.y) * f0,
         z: p0.z + (p1.z - p0.z) * f0,
         u: p0.u + (p1.u - p0.u) * f0,
         v: p0.v + (p1.v - p0.v) * f0
        },
        {x: p0.x + (p1.x - p0.x) * f1,
         y: p0.y + (p1.y - p0.y) * f1,
         z: p0.z + (p1.z - p0.z) * f1,
         u: p0.u + (p1.u - p0.u) * f1,
         v: p0.v + (p1.v - p0.v) * f1
        }
    ];
    return ps;
}

// for debugging
function drawPerspectiveTriUnclippedSubX(c3d, v0, tv0, v1, tv1, v2, tv2) {
  var ctx = c3d.canvas_ctx_;
  ctx.beginPath();
  ctx.moveTo(tv0.x, tv0.y);
  ctx.lineTo(tv1.x, tv1.y);
  ctx.lineTo(tv2.x, tv2.y);
  ctx.lineTo(tv0.x, tv0.y);
  ctx.stroke();
}
  

function drawPerspectiveQuadUnclippedSub(c3d, v0, tv0, v1, tv1, v2, tv2, v3, tv3, depth_count) {
  var edgelen01 =
    Math.abs(tv0.x - tv1.x) +
    Math.abs(tv0.y - tv1.y);
  var edgelen12 =
    Math.abs(tv1.x - tv2.x) +
    Math.abs(tv1.y - tv2.y);
  var edgelen23 =
    Math.abs(tv2.x - tv3.x) +
    Math.abs(tv2.y - tv3.y);
  var edgelen30 =
    Math.abs(tv3.x - tv0.x) +
    Math.abs(tv3.y - tv0.y);
  var zdepth01 =
    Math.abs(v0.z - v1.z);
  var zdepth12 =
    Math.abs(v1.z - v2.z);
  var zdepth23 =
    Math.abs(v2.z - v3.z);
  var zdepth30 =
    Math.abs(v3.z - v0.z);

  var subdiv = (edgelen01 * zdepth01 > options.subdivide_factor) ||
      (edgelen12 * zdepth12 > options.subdivide_factor) ||
      (edgelen23 * zdepth23 > options.subdivide_factor) ||
      (edgelen30 * zdepth30 > options.subdivide_factor);

  if (depth_count) {
    depth_count--;
    if (depth_count == 0) {
      subdiv = false;
    } else {
      subdiv = true;
    }
  }

    // v0       v1
    //   **----*
    //   *     |
    //   |     *
    //   *----**
    // v3       v2
    //
    

  if (!subdiv) {
    if (draw_wireframe) {
      var ctx = c3d.canvas_ctx_;
      ctx.beginPath();
      ctx.moveTo(tv0.x, tv0.y);
      ctx.lineTo(tv1.x, tv1.y);
      ctx.lineTo(tv2.x, tv2.y);
      ctx.lineTo(tv3.x, tv3.y);
      ctx.lineTo(tv0.x, tv0.y);
      ctx.stroke();
    } else {
      jsgl.drawQuad(c3d.canvas_ctx_, images[0],
                        tv0.x, tv0.y,
                        tv1.x, tv1.y,
                        tv2.x, tv2.y,
                        tv3.x, tv3.y,
                        v0.u, v0.v,
                        v1.u, v1.v,
                        v2.u, v2.v);
    }
    return;
  }

  // Need to subdivide.
  var [v01a, v01b] = bisectFat(v0, v1, edgelen01);
  var [v12a, v12b] = bisectFat(v1, v2, edgelen12);
  var [v23a, v23b] = bisectFat(v2, v3, edgelen23);
  var [v30a, v30b] = bisectFat(v3, v0, edgelen30);
  var [vCCa, vCCb] = bisectFat(v01a, v23b, edgelen12);
  var [vCCc, vCCd] = bisectFat(v23a, v01b, edgelen30);

  var [tv01a, tv01b] = [jsgl.projectPoint(v01a), jsgl.projectPoint(v01b)];
  var [tv12a, tv12b] = [jsgl.projectPoint(v12a), jsgl.projectPoint(v12b)];
  var [tv23a, tv23b] = [jsgl.projectPoint(v23a), jsgl.projectPoint(v23b)];
  var [tv30a, tv30b] = [jsgl.projectPoint(v30a), jsgl.projectPoint(v30b)];
  var [tvCCa, tvCCb] = [jsgl.projectPoint(vCCa), jsgl.projectPoint(vCCb)];
  var [tvCCc, tvCCd] = [jsgl.projectPoint(vCCc), jsgl.projectPoint(vCCd)];

    drawPerspectiveQuadUnclippedSub(c3d, v0, tv0, v01a, tv01a, vCCa, tvCCa, v30b, tv30b, depth_count);
    drawPerspectiveQuadUnclippedSub(c3d, v1, tv1, v12a, tv12a, vCCd, tvCCd, v01b, tv01b, depth_count);
    drawPerspectiveQuadUnclippedSub(c3d, v2, tv2, v23a, tv23a, vCCc, tvCCc, v12b, tv12b, depth_count);
    drawPerspectiveQuadUnclippedSub(c3d, v3, tv3, v30a, tv30a, vCCb, tvCCb, v23b, tv23b, depth_count);
}

function drawPerspectiveQuadUnclipped(c3d, v0, v1, v2, v3, depth_count) {
  var tv0 = jsgl.projectPoint(v0);
  var tv1 = jsgl.projectPoint(v1);
  var tv2 = jsgl.projectPoint(v2);
  var tv3 = jsgl.projectPoint(v3);
  drawPerspectiveQuadUnclippedSub(c3d, v0, tv0, v1, tv1, v2, tv2, v3, tv3, depth_count);
}


function drawPerspectiveQuad(c3d, v0, v1, v2, v3, depth_count) {
    var clip =
        ((v0.z < MIN_Z) ? 1 : 0) +
        ((v1.z < MIN_Z) ? 2 : 0) +
        ((v2.z < MIN_Z) ? 4 : 0) +
        ((v3.z < MIN_Z) ? 8 : 0);
    if (clip == 0) {
        // No verts need clipping.
        drawPerspectiveQuadUnclipped(c3d, v0, v1, v2, v3, depth_count);
        return;
    }
    if (clip == 15) {
        // All verts are behind the near plane; don't draw.
        return;
    }

  var min_z2 = MIN_Z * 3;
  var clip2 = ((v0.z < min_z2) ? 1 : 0) +
              ((v1.z < min_z2) ? 2 : 0) +
              ((v2.z < min_z2) ? 4 : 0) + 
              ((v3.z < min_z2) ? 8 : 0);
  if (clip2 == 15) {
    // All verts are behind the guard band, don't recurse.
    return;
  }

    var approxSize = (
        Math.abs(v0.x - v2.x) + Math.abs(v0.y - v2.y) +
        Math.abs(v1.x - v3.x) + Math.abs(v1.y - v3.y)) / 2;
    var [v01a, v01b] = bisectFat(v0, v1, approxSize);
    var [v12a, v12b] = bisectFat(v1, v2, approxSize);
    var [v23a, v23b] = bisectFat(v2, v3, approxSize);
    var [v30a, v30b] = bisectFat(v3, v0, approxSize);
    var [vCCa, vCCb] = bisectFat(v01a, v23b, approxSize);
    var [vCCc, vCCd] = bisectFat(v23a, v01b, approxSize);

  if (depth_count) {
    depth_count--;
      if (depth_count == 0) {
          return;
      }
  }

    drawPerspectiveQuad(c3d, v0, v01a, vCCa, v30b, depth_count);
    drawPerspectiveQuad(c3d, v1, v12a, vCCd, v01b, depth_count);
    drawPerspectiveQuad(c3d, v2, v23a, vCCc, v12b, depth_count);
    drawPerspectiveQuad(c3d, v3, v30a, vCCb, v23b, depth_count);
}

function draw() {
  // Clear canvas.
  var ctx = c3d.canvas_ctx_;

  ctx.globalAlpha = options.whiteout_alpha;
  ctx.fillStyle = '#000000';
  ctx.fillRect(0, 0, canvas_elem.width, canvas_elem.height);
  ctx.globalAlpha = 1;

  var view_mat = jsgl.makeViewFromOrientation(camera_mat);

  // Update transform.
  jsgl.multiplyAffineTo(proj_mat, view_mat, temp_mat0);
  jsgl.multiplyAffineTo(temp_mat0, object_mat, temp_mat1);
  c3d.setTransform(temp_mat1);

  // Draw.
  var im_width = images[0].width;
  var im_height = images[0].height;
  var verts = [
	  {x:-1, y:-1, z: 0, u:0, v:0},
	  {x: 1, y:-1, z: 0, u:im_width, v:0},
	  {x: 1, y: 1, z: 0, u:im_width, v:im_height},
	  {x:-1, y: 1, z: 0, u:0, v:im_height}
	  ];
  var tverts = [];
  for (var i = 0; i < verts.length; i++) {
    tverts.push(jsgl.transformPoint(c3d.transform_, verts[i]));
    tverts[i].u = verts[i].u;
    tverts[i].v = verts[i].v;
  }

  if (options.draw_quad) {
      drawPerspectiveQuad(c3d, tverts[0], tverts[1], tverts[2], tverts[3], options.nonadaptive_depth);
  } else {
      drawPerspectiveTri(c3d, tverts[0], tverts[1], tverts[2], options.nonadaptive_depth);
      drawPerspectiveTri(c3d, tverts[0], tverts[2], tverts[3], options.nonadaptive_depth);
  }

  if (options.wireframe) {
    ctx.globalAlpha = 0.3;
    ctx.fillRect(0, 0, canvas_elem.width, canvas_elem.height);
    draw_wireframe = true;
    ctx.globalAlpha = 1;
    if (options.draw_quad) {
      drawPerspectiveQuad(c3d, tverts[0], tverts[1], tverts[2], tverts[3], options.nonadaptive_depth);
    } else {
      drawPerspectiveTri(c3d, tverts[0], tverts[1], tverts[2], options.nonadaptive_depth);
      drawPerspectiveTri(c3d, tverts[0], tverts[2], tverts[3], options.nonadaptive_depth);
    }
    draw_wireframe = false;
  }
}

const SeedableRandom = require("./random");
const seedableRandom = new SeedableRandom();
seedableRandom.seed(125);

function rotateObjectRandom(angleVariation) {
    const getRandom = (lower, upper) => (seedableRandom.next() * (upper-lower) + lower);
    let x_deg = getRandom(-angleVariation, angleVariation);
    let y_deg = getRandom(-angleVariation, angleVariation);
    let z_deg = getRandom(0, 359);
    rotateObjectXYZ(x_deg, y_deg, z_deg);
}

function rotateObjectXYZ(x_deg, y_deg, z_deg) {
    let x_rad = x_deg * Math.PI / 180;
    let y_rad = y_deg * Math.PI / 180;
    let z_rad = z_deg * Math.PI / 180;
    rotateObject({x:x_rad, y:0, z:0});
    rotateObject({x:0, y:y_rad, z:0});
    rotateObject({x:0, y:0, z:z_rad});
}

function rotateObject(scaled_axis) {
  var angle = Math.sqrt(jsgl.dotProduct(scaled_axis, scaled_axis));

  var axis = jsgl.vectorNormalize(scaled_axis);
  var mat = jsgl.makeRotateAxisAngle(axis, angle);
  object_mat = jsgl.multiplyAffine(mat, object_mat);
  jsgl.orthonormalizeRotation(object_mat);
}

function init(canvas_id, angleVariation) {
    return new Promise(resolve => {
        canvas_elem = document.getElementById(canvas_id);
        var ctx = canvas_elem.getContext('2d');
    
        c3d = new jsgl.Context(ctx);
    
        temp_mat0 = jsgl.makeIdentityAffine();
        temp_mat1 = jsgl.makeIdentityAffine();
        temp_mat2 = jsgl.makeIdentityAffine();
        proj_mat = jsgl.makeWindowProjection(
            canvas_elem.width, canvas_elem.height, horizontal_fov_radians);
        object_mat = jsgl.makeOrientationAffine(
            {x:0, y:0, z:0}, {x:1, y:0, z:0}, {x:0, y:1, z:0});
        camera_mat = jsgl.makeOrientationAffine(
            {x:0, y:0, z: -0.2 - target_distance}, {x:0, y:0, z:1}, {x:0, y:-1, z:0});
    
        rotateObjectRandom(angleVariation);
        requestAnimationFrame(() => {
            draw();
            resolve();
        });
    });
}

// Awesome cubemaps: http://www.humus.name/index.php?page=Textures&&start=32

export function generate_perspective_with_image_src(canvas_id, src, angleVariation) {
    return new Promise(resolve => {
        images.push(new Image());
        images[0].onload = function () {
            init(canvas_id, angleVariation)
                .then(resolve);
        };
        images[0].src = src;
    });
}