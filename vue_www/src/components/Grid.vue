<template>
    <div class="font-size-class">
        <div :class="{ori: is_ori, not_ori: !is_ori}">
            {{ grid_value > 0 ? grid_value : " " }}
        </div>
    </div>
</template>

<script setup>
const props = defineProps({
    grid_value: {
        required: true,
    },
    is_ori: {
        required: true,
    }
});




</script>

<style>
.font-size-class {
    display: flex;
    font-size: 25px;
    /* align-content:   center;  */
    justify-content: center;
    align-items: center;
}



.ori {
    color: red;
}

.not_ori {
    color: blue;
}
</style>
