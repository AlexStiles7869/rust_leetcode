<script setup lang="ts">
import { onMounted, computed, ComputedRef, ref, Ref } from 'vue'
import IslandTiles from './components/IslandTiles.vue'

import { gsap } from 'gsap'
import { ExpoScaleEase } from 'gsap/EasePack'

gsap.registerPlugin(ExpoScaleEase)

onMounted(() => {
  const timeline = gsap.timeline()
  timeline
    .to('.loader', {
      delay: 0.5,
      height: '0%',
      duration: 1,
      ease: 'expo.out',
    })
    .to(
      '.landing-text h1',
      {
        opacity: 1,
        filter: 'blur(0px)',
        duration: 0.65,
        y: '0%',
        transformOrigin: 'center center',
        rotationX: 0,
        rotationZ: 0,
        ease: 'power4.out',
        stagger: {
          each: 0.075,
        },
      },
      '<20%',
    )
})

const copyState: Ref<string> = ref('')
const copyButtonText: ComputedRef<string> = computed(() => {
  let buttonText: string = 'Copy Islands'
  switch (copyState.value) {
    case '':
      buttonText = 'Copy Islands'
      break
    case 'pending':
      buttonText = 'Copying'
      break
    case 'copied':
      buttonText = 'Copied to Clipboard'
      break
    default:
      break
  }
  return buttonText
})

const copyIslandToClipboard = () => {
  // Set copy pending flag to update button state to show loader
  copyState.value = 'pending'

  navigator.clipboard.writeText(window.location.href).then(
    () => {
      // Wait 500ms before reverting back to default state
      setTimeout(() => {
        copyState.value = 'copied'
        setTimeout(() => {
          copyState.value = ''
        }, 1000)
      }, 1000)
    },
    () => {
      console.log(':(')
    },
  )
}
</script>

<template>
  <!-- Navigation bar -->
  <div class="loader"></div>
  <nav>
    <div class="logo">
      <span>AJ</span>
    </div>
  </nav>
  <div class="app-container">
    <div class="landing-bar">
      <div class="landing-text">
        <h1>Islands</h1>
        <p>A simple demo of Vue and Rust WebAssembly.</p>
      </div>
      <div class="copy-button-container">
        <button @click="copyIslandToClipboard" :class="[copyState]">
          <!-- Copy Icon -->
          <span class="material-symbols-outlined"> content_copy </span>
          <span>{{ copyButtonText }}</span>
        </button>
      </div>
    </div>
    <IslandTiles></IslandTiles>
    <div class="world-info">
      <h2>World Stats</h2>
      <div class="world-stats">
        <div class="world-stat">
          <h3>Number of Islands</h3>
          <span class="main">0</span>
        </div>
        <div class="world-stat">
          <h3>Size of Largest Island</h3>
          <span
            ><span class="main">0</span><span class="sec"> Units<sup>2</sup></span></span
          >
        </div>
        <div class="world-stat">
          <h3>Number of Tiles</h3>
          <span><span class="main">0</span><span class="sec"> of 20 spots</span></span>
        </div>
      </div>
    </div>
  </div>
  <!-- NOTE: Other island information -> Number of discrete islands, size of largest island, number of island tiles -->
  <!-- Footer bar -->
</template>

<style scoped>
.loader {
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: black;
  bottom: 0;
  z-index: 9999;
}
nav {
  margin: 2rem;

  .logo {
    padding: 0.75rem;
    display: inline-block;
    background-color: black;
    width: 4.5rem;
    height: 4.5rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.125rem;
    user-select: none;

    span {
      font-size: 2rem;
      color: white;
      font-weight: 700;
    }
  }
}
.app-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin: 0 auto;
  width: fit-content;

  > * {
    width: 100%;
  }
}
.landing-bar {
  display: flex;
  gap: 2rem;
  justify-content: space-between;
  align-items: stretch;
  margin-bottom: 1.5rem;

  .landing-text {
    overflow: hidden;
    perspective: 14px;
    h1 {
      transform: translateY(-100%) rotateZ(-14deg) rotateX(1deg);
      opacity: 0;
      filter: blur(20px);
      font-size: 3rem;
      letter-spacing: -0.125rem;
      font-weight: 600;
      margin-bottom: 0.75rem;
    }
    p {
      color: #333;
    }
  }
  .copy-button-container {
    button {
      border: none;
      height: 100%;
      border-radius: 1rem;
      background-color: black;
      padding: 1rem 1rem;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-end;
      gap: 0.25rem;
      width: calc-size(auto);
      cursor: pointer;
      transition:
        transform 200ms,
        background-color 200ms;
      overflow: hidden;
      position: relative;

      > * {
        z-index: 2;
      }

      &::after {
        display: block;
        position: absolute;
        height: 100%;
        width: 100%;
        z-index: 1;
        background-color: green;
      }

      &.copied {
        background-color: green;
      }
      &:hover {
        /* transform: scale(1.05); */
        /* * { */
        /* transform: translateY(-0.2rem); */
        /* } */
      }

      /* * { */
      /* transition: transform 200ms; */
      /* } */

      .material-symbols-outlined {
        font-size: 2.125rem;
        font-variation-settings: 'wght' 300;
      }

      span {
        font-size: 0.8rem;
        color: white;
        font-weight: 500;
      }
    }
  }
}

.world-info {
  margin-top: 2rem;
  h2 {
    font-size: 1.75rem;
    font-weight: 500;
  }

  .world-stats {
    margin-top: 1rem;
    display: flex;
    border-radius: 1rem;
    border: 2px solid #ddd;

    & > *:not(:first-child) {
      border-left: 2px solid #ddd;
    }

    .world-stat {
      flex-basis: 33.3%;
      padding: 1rem;

      h3 {
        font-size: 1rem;
        font-weight: 400;
        margin-bottom: 0.5rem;
      }

      span.main {
        font-size: 1.5rem;
        font-weight: 500;
      }

      span.sec {
        font-size: 0.9rem;
        color: #555;
      }
    }
  }
}
</style>
