```vue
<script lang="ts"   setup >
import {

    ref 
} from "vue"


    const count   = ref(1)
    function add (a:number,b:number):number { 
                return a +b 
        }   </script> 


<template>  
    <button  @click="()=> count = add(count,count )">Increment </button>
        </template>


```
