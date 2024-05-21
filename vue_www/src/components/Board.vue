<script setup>
import { defineComponent, onMounted, reactive, watch } from "vue";

import { Soduku } from "my_wasm";
import { useMessage, createDiscreteApi, NPopover, NButton, NSlider, NSwitch } from "naive-ui";
import Grid from "@/components/Grid.vue";

let soduku = Soduku.new();

let data = reactive({
    rows: 9,
    columns: 9,
    numbers: [1, 22, 333],
    counter: 0,
    fill_row: -1,
    fill_col: -1,

    candiList: null,
    warn1: "原始位置不可修改",
    warn2: "陷入死局 请回退",   // todo 检查一下是否已经下过这个位置
    fillCooList: [],   // 已经填写的
    diffcu: 25,
    digList: [],
    digDict: {

    },
    ans_numbers: [],
    ans_string: "",
    aux_mode: true,
    msg: "",
});

const { message } = createDiscreteApi(["message"]);

//

function handleUpdateShow(rowIndex, colIndex) {
    console.log("", rowIndex, colIndex);

    // 原始棋盘的棋子: 不可选择.
    // 已经没有选项了: 不可选择.


    if (!data.aux_mode) {

        data.candiList = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    } else {

        if (soduku.is_in_ori_board(rowIndex, colIndex)) {
            data.candiList = [];
        } else {

            data.candiList = soduku.get_candi_list(rowIndex, colIndex);

        }


    }


    if (
        !soduku.is_in_ori_board(rowIndex, colIndex) &&
        data.candiList.length > 0
    ) {
        console.log("候选列表: ", data.candiList);

        data.fill_row = rowIndex;
        data.fill_col = colIndex;
    } else if (soduku.is_in_ori_board(rowIndex, colIndex)) {
        console.log("不可修改初始棋盘!");
        // message.warning(data.warn1);
        data.msg = data.warn1;
    } else if (data.candiList.length === 0) {

        if (!soduku.is_setted(rowIndex, colIndex)) {
            console.log("已经没有可能性");
            message.warning(data.warn2);
        }

    }

}



function fill_in(rowIndex, colIndex, number) {
    soduku.input_a_number(rowIndex, colIndex, number);
    data.numbers = soduku.get_current_board();
}

function handleClickGetAnswer() {
    console.log("看答案");


    let answer = soduku.get_answer();

    console.log(answer);
}
function handleFillInClick(index) {
    console.log("确认落子");

    if (data.fill_row >= 0 && data.fill_col >= 0) {
        // 执行填空
        fill_in(data.fill_row, data.fill_col, index);

        // 记录 落子的坐标  方便反悔
        data.fillCooList.push([data.fill_row, data.fill_col]);
        data.counter += 1;
    }

    // 重置
    data.fill_row = -1;
    data.fill_col = -1;


}
function handleClickGenNew() {
    soduku = Soduku.new();
    data.numbers = soduku.get_current_board();
    data.fillCooList = [];
}
function handleClickRestart() {
    soduku.restart();
    data.numbers = soduku.get_current_board();
    // console.log("")
    data.fillCooList = [];
}


function handleClickBack() {


    if (data.fillCooList.length > 0) {
        console.log("回退");
        var last_coo = data.fillCooList.pop();
        data.counter -= 1;
        soduku.back(last_coo[0], last_coo[1]);

        data.numbers = soduku.get_current_board();
    } else {

        console.log("回退失败", data.fillCooList, data.fillCooList.length)
        message.error("无路可退 🈚🈚️️️🈚️️️🈚️️️🈚️️️️️️️")
    }


}



onMounted(() => {

    data.numbers = soduku.get_new_board();

    data.digList = soduku.get_dig_list();
    data.ans_numbers = soduku.get_answer_numbers();
    data.ans_string = soduku.get_answer();
    data.diffcu = data.digList.length;

    data.digDict[1] = "挖去更少";
    data.digDict[data.digList.length] = "挖去更多";
});




watch(() => data.diffcu, (newDiffcu, oldDiffcu) => {
    console.log("新的难度是:", newDiffcu, oldDiffcu);


    // 调整本局难度:  需要的数据都在答案里.


    if (newDiffcu < data.digList.length) {

        data.numbers = soduku.get_current_board_change_diffcu(newDiffcu);

    }



})

watch(() => data.counter, (newCounter, oldCounter) => {
    if (newCounter === data.diffcu) {
        message.success("你胜利了! ✌️✌️✌️✌️✌️")
    }
})


</script>

<template>
    <div>
        <div v-for="(row, rowIndex) in data.rows" :key="rowIndex" class="row">
            <div v-for="(column, colIndex) in data.columns" :key="colIndex" class="cell">
                <n-popover placement="top" trigger="hover" @update:show="handleUpdateShow(rowIndex, colIndex)">
                    <template #trigger>
                        <Grid class="grid" :grid_value="data.numbers[rowIndex * 9 + colIndex]"
                            :is_ori="soduku.is_in_ori_board(rowIndex, colIndex)"></Grid>
                    </template>
                    <div class="candi" v-for="index in data.candiList" @click="handleFillInClick(index)">

                        <Grid class="grid" :grid_value="index" :is_ori="soduku.is_in_ori_board(rowIndex, colIndex)">
                        </Grid>

                    </div>
                </n-popover>
            </div>
        </div>


        <div>

            <n-button @click="handleClickBack"> 回退 </n-button>



            <n-popover trigger="hover">
                <template #trigger>
                    <n-button @click="handleClickGetAnswer">看答案</n-button>
                </template>


                <div class="enter_wrapper">

                    {{ data.ans_string }}
                </div>


            </n-popover>


            <n-button @click="handleClickRestart">重开本局</n-button>
            <n-button @click="handleClickGenNew">生成新局</n-button>



            <br />
            辅助模式:
            <n-switch v-model:value="data.aux_mode">
                <template #checked>
                    帮助你排除一些数字
                </template>
                <template #unchecked>
                    现在需要自己思考
                </template>
            </n-switch>

            <br />
            调整本局难度:
            <n-slider v-model:value="data.diffcu" :marks="data.digDict" :step="1" :min="1" :max="81" />

        </div>



    </div>
</template>

<style>
.row {
    display: flex;
}

.cell {
    width: 49px;
    height: 49px;
    border: 1px solid black;
    display: flex;
    justify-content: center;
    align-items: center;
}

.grid {
    width: 40px;
    height: 40px;
    background-color: antiquewhite;
}

.cadi {
    display: flex;
    font-size: 400px;
}

.enter_wrapper {
    white-space: pre-wrap;
}
</style>
