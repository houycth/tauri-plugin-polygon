<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { polygon } from 'tauri-plugin-polygon-api';
// A small JavaScript library to create and animate annotations on a web page.
// https://roughnotation.com/
import { annotate, annotationGroup } from 'rough-notation';

const note_el = ref();
const demo_el = ref();
const plugin_el = ref();
const ignore_el = ref();
const dclick_el = ref();
const response_el = ref();
const click_el = ref();

const btnShow = ref(false);
// Unit: vw
const btnSize = ref(1.5);
const btnPos = ref({
  left: 0,
  top: 0,
})
const selectedContent = ref("");

onMounted(() => {
  let ag = annotationGroup([
    annotate(note_el.value, { type: 'highlight', color: '#e06c75' }),
    annotate(demo_el.value, { type: 'box', color: '#61afef' }),
    annotate(plugin_el.value, { type: 'circle', color: '#d17c44' }),
    annotate(ignore_el.value, { type: 'underline', color: '#e65100' }),
    annotate(dclick_el.value, { type: 'underline', color: '#e65100' }),
    annotate(response_el.value, { type: 'underline', color: '#e65100' }),
    annotate(click_el.value, { type: 'underline', color: '#e65100' }),
  ]);
  ag.show();

  // We have several events that we can listen to: LeftClick, RightClick, DoubleClick, MouseMove, Wheel, Drag and Error.
  // Note that the click/drag events are only triggered in unregistered areas.
  // So it might be a good idea to trigger our program by them.
  // As for registered area, we can handle it in frontend.
  polygon.on("DoubleClick", async payload => {
    ag.hide();
    ag.show();

    const content = await invoke("get_content");

    if (content && (isCamelCase(content) || isSnakeCase(content))) {
      selectedContent.value = content;

      let x = payload.position.x;
      let y = payload.position.y;

      let size = btnSize.value / 100 / 2;
      // We've already registered a polygon with id `BUTTON` in rust, so we can use it directly.
      // Here we make cursor the center of our button.
      // Note:
      // 1. Percentage is used here.
      // 2. At least 3 points needed.
      // 3. Order of points matters.
      await polygon.update("BUTTON", [
        [x - size, y - size],
        [x + size, y - size],
        [x + size, y + size],
        [x - size, y + size],
      ])
      // Do forget to make the polygon 'visible' (We do not really see the polygon).
      await polygon.show("BUTTON")
      // Show button in frontend.
      btnShow.value = true;
      // The position we got from backend is parcentage, so we need to convert it to pixel.
      // Since we make the window fullscreen, we can use window.screen.width.
      btnPos.value = {
        left: (payload.position.x - size) * window.screen.width,
        top: (payload.position.y - size) * window.screen.width,
      }
    }
  })

  polygon.on("LeftClick", async payload => {
    btnShow.value = false;
    await polygon.hide("BUTTON")
  })

  polygon.on("Wheel", async payload => {
    btnShow.value = false;
    await polygon.hide("BUTTON")
  })
})

// Hide polygon. Hide UI. Double click to get back focus and select the word. Chang it. Done.
async function handleClick() {
  await polygon.hide("BUTTON");
  btnShow.value = false;
  await invoke("click_to_back_focus");
  const convertedContent = convert(selectedContent.value);
  await invoke("set_content", {content: convertedContent});
}

function isSnakeCase(str) {
  const snakeCaseRegex = /^[a-z]+(_[a-z]+)*$/;
  return snakeCaseRegex.test(str);
}

function isCamelCase(str) {
  const camelCaseRegex = /^[a-z][A-Za-z]*$/;
  return camelCaseRegex.test(str);
}

function snakeToCamel(str) {
  return str.replace(/(_\w)/g, function (m) {
    return m[1].toUpperCase();
  });
}

function camelToSnake(str) {
  return str.replace(/([A-Z])/g, function (m) {
    return "_" + m.toLowerCase();
  });
}

function convert(content) {
  if (isCamelCase(content)) {
    return camelToSnake(content);
  } else if (isSnakeCase(content)) {
    return snakeToCamel(content);
  }
}
</script>

<template>
  <main class="container">
    <div class="note">
      <div>
        <span ref="note_el">Note: </span>This application is a <span ref="demo_el">demo</span> for <span
          ref="plugin_el">tauri-plugin-polygon</span>.
      </div>
      <div class="script">
        <p>1. These texts would <span ref="ignore_el">ignore mouse event</span>.</p>
        <p>2. <span ref="dclick_el">Double click</span> to select a snake/camel like words in any editor to create a <span
            ref="response_el">responsive area</span>.</p>
        <p>3. Then <span ref="click_el">click the displayed button</span> to convert to camel/snake.</p>
      </div>
    </div>
    <button @click="handleClick" :class="btnShow ? 'convertBtn show' : 'convertBtn hide'"
      :style="{ width: btnSize + 'vw', height: btnSize + 'vw', left: btnPos.left + 'px', top: btnPos.top + 'px' }">
      <svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 24 24">
        <path fill="currentColor"
          d="M12 23q-2.8 0-5.15-1.275T3 18.325V21H1v-6h6v2H4.525q1.2 1.8 3.163 2.9T12 21q1.875 0 3.513-.713t2.85-1.924q1.212-1.213 1.925-2.85T21 12h2q0 2.275-.863 4.275t-2.362 3.5q-1.5 1.5-3.5 2.363T12 23ZM1 12q0-2.275.863-4.275t2.362-3.5q1.5-1.5 3.5-2.362T12 1q2.8 0 5.15 1.275t3.85 3.4V3h2v6h-6V7h2.475q-1.2-1.8-3.163-2.9T12 3q-1.875 0-3.513.713t-2.85 1.924Q4.426 6.85 3.714 8.488T3 12H1Zm11 5l-1.55-3.45L7 12l3.45-1.575L12 7l1.575 3.425L17 12l-3.425 1.55L12 17Z" />
      </svg>
    </button>
  </main>
</template>

<style scoped></style>
<style>
* {
  margin: 0;
  padding: 0;
}

main.container {
  width: 100vw;
  height: 100vh;
  background-color: transparent;
}

div.note {
  position: absolute;
  top: 15%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 2rem;
  font-weight: bold;
  color: #90a4ae;
}

div.script {
  margin-top: 2vh;
  text-align: center;
  font-size: 1rem;
  line-height: 2rem;
}

.convertBtn {
  position: absolute;
  background-color: transparent;
  border: 0;
  outline: none;
  color: #e64a19;
  border-radius: 50%;
  box-sizing: content-box;
  cursor: pointer;
}

.convertBtn.show {
  opacity: 1;
  transition: opacity 0.3s ease-in-out;
}

.convertBtn.hide {
  opacity: 0;
  transition: opacity 0.3s ease-in-out;
}

.convertBtn svg {
  width: 100%;
  height: 100%;
  transition: all 0.5s ease-in-out;
}

.convertBtn svg:hover {
  rotate: 360deg;
  transition: all 0.5s ease-in-out;
}
</style>
