<template>
  <div id="header_pane">
      <h1>{{msg}}</h1>
  </div>
</template>

<script>
export default {
  name: 'HeaderPane',
  props: {
      msg: String
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>