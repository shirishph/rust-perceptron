<template>
  <div id="app">
      <HeaderPane :msg="this.msg"/>
  </div>
</template>

<script>
import HeaderPane from './components/HeaderPane.vue'
import axios from 'axios'
export default {
  name: 'App',
  components: {
    HeaderPane
  },
  data() {
      return {
          msg: "test"
      }
  },
  methods: {
    fetchData: function() {
        console.log("master/fetching data")
        axios
        .get('http://localhost:8000/base')
        .then(response => {
            console.log('master/response', response.data)
            this.msg = response.data
        })
        .catch(error => {
            console.log(error)
        })
      }
    },
    mounted() {
      console.log("master/mounted")
      this.fetchData()
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
