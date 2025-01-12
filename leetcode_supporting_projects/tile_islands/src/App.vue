<script setup lang="ts">
import { computed, ComputedRef, ref, Ref } from 'vue'
import IslandTiles from './components/IslandTiles.vue'

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
  <!-- NOTE: Other island information -> Number of discrete islands, size of largest island, number of island tiles -->
  <!-- Footer bar -->
</template>

<style scoped>
.landing-bar {
  display: flex;
  gap: 2rem;
  justify-content: space-between;
  align-items: stretch;
  margin-bottom: 1.5rem;

  .landing-text {
    h1 {
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
