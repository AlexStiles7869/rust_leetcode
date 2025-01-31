<script setup lang="ts">
const tiles: string[][] = [
  ['1', '1', '1', '1', '0'],
  ['1', '1', '0', '1', '0'],
  ['1', '1', '0', '0', '0'],
  ['0', '0', '0', '0', '0'],
]

window.location.hash = tiles.reduce(
  (prev, curr) =>
    prev.concat(
      curr.reduce((prev, curr) => prev + curr),
      '',
    ),
  '',
)

const changeTile = (evt: MouseEvent) => {
  if (evt.currentTarget == null) return

  const target = evt.currentTarget as HTMLElement

  const tile_location: [number, number] = JSON.parse(target.dataset.index!)
  const [row, column]: [number, number] = tile_location

  tiles[row][column] = '1'
}
</script>

<template>
  <div class="tile-container">
    <div v-for="(tile_row, i) in tiles" class="tile-row" :key="i">
      <div
        v-for="(_tile, j) in tile_row"
        class="tile"
        :data-index="`[${i}, ${j}]`"
        :key="j"
        @click="changeTile"
      >
        <div class="tile-background"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tile-container {
  display: flex;
  flex-direction: column;
  border-radius: 2rem;
  overflow: hidden;
  background-color: #ddd;

  .tile-row {
    display: flex;

    .tile {
      height: 8rem;
      width: 8rem;
      background-color: #ddd;
      cursor: pointer;
      display: flex;
      justify-content: center;
      align-items: center;

      .tile-background {
        width: 1rem;
        height: 1rem;
        transform: scale(7);
        background-position: 0 0;
        image-rendering: pixelated;
        image-rendering: crisp-edges;
        transition: transform 200ms;

        &.grass {
          /* transform: scale(8); */
          background-image: url('@/assets/grass_3.png');
        }
      }

      &:not(.grass):hover {
        border-radius: 2rem;
        background-color: #ccc;
        z-index: 2;
        border: 4px dotted #bbb;
      }
    }
  }
}
</style>
