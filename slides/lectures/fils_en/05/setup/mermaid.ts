import { defineMermaidSetup } from '@slidev/types'

export default defineMermaidSetup(() => {
  return {
    // theme: 'forest',
    sequence: {
      diagramMarginX: 30,
      diagramMarginY: 10,
      boxTextMargin: 5,
      noteMargin: 10,
      messageMargin: 3,
      mirrorActors: true,
      height: 50
    }
  }
})
