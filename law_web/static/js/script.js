document.getElementById('search-law-form').addEventListener('submit', async (event) => {
    event.preventDefault();
    const chapter = $("#chapter").val();
    const num = $("#num").val();

    const response = await fetch(`http://localhost:9090/questions/${chapter}/${num}`);
    const tableHtml = await response.text();

    const tableContainer = document.getElementById('result-area');
    tableContainer.innerHTML = tableHtml;// 清空表格

    const question = { chapter: chapter, num: num };

    await fetch('http://localhost:9090/questions', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(question),
    });


    // 清除表單
    event.target.reset();// 重新載入問題列表

    loadQuestions();
});

async function loadQuestions() {
    const response = await fetch('http://localhost:9090/records_to_laws');
    const tableHtml = await response.text();

    const tableContainer = document.getElementById('record_table');
    tableContainer.innerHTML = tableHtml;// 清空表格

}




loadQuestions();





$(function(){
    $("#j-button").click(function () {
        $(this).html("Click").css("background", "#000033");
    });
});

document.getElementById('search-btn').addEventListener('click', () => {
    document.getElementById('search-area').style.display = 'flex';
    document.getElementById('record-area').style.display = 'none';
    document.getElementById('test-area').style.display = 'none';
});

document.getElementById('record-btn').addEventListener('click', () => {
    document.getElementById('search-area').style.display = 'none';
    document.getElementById('record-area').style.display = 'flex';
    document.getElementById('test-area').style.display = 'none';
});

document.getElementById('test-btn').addEventListener('click', () => {
    document.getElementById('search-area').style.display = 'none';
    document.getElementById('record-area').style.display = 'none';
    document.getElementById('test-area').style.display = 'flex';
});

$(function() {
    // 使用事件委託，將事件處理程序附加到父元素上
    $("#record_table").on("click", ".record-button", async function () {
        $("#show-words").remove();
        $("#show-words").fadeOut(1000);
        const buttonText = $(this).text();
        const [element1, element2] = buttonText.split('-');
        const response = await fetch(`http://localhost:9090/questions/${element1}/${element2}`);
        const tableHtml = await response.text();
        $("#record_show").append(`<div id='show-words' style='display: none;'>${tableHtml}</div>`);
        $("#show-words").fadeIn(1000);
    });
});

function load_all_lines(chapter) {
    $.ajax({
        url: `http://localhost:9090/questions/all_lines/${chapter}`,
        method: 'GET',
        success: function(response) {
            // 將回應內容加入到 #all-lines
            $("#all-lines").append(`<div id='all-chapters-ul' style='display: none;'>${response}</div>`);

            // 動態加入 CSS 樣式
            $("<style>")
                .prop("type", "text/css")
                .html(`
                    .law-content {
                        border-bottom: 1px solid white;
                        margin-bottom: 20px;
                        padding: 10px 0;
                    }
                    .law-chapter {
                        font-weight: bold;
                        text-align: left;
                    }
                    .law-lines {
                        text-align: left;
                    }
                    .chapter-li a:hover {
                        background-color: #111;
                    }
                `)
                .appendTo("head");

            $("#all-chapters-ul").fadeIn(1000);
        },
        error: function(xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}

$(document).ready(function() {
    $("#law-search-area-form").submit(function(event) {
        event.preventDefault();
        const chapter = $("#search-chapter").val();
        $("#law-search-area").css("display", "none");
        $("#show-type-nav").css("display", "block");
        $(".chapter_is").html(chapter);
        load_all_search_chapters(chapter);
        load_all_lines(chapter);
        event.target.reset();
    });
});

$("#show-type-chapter").click(function (){
    $("#law-show-area").css("display", "block");
    $("#show-type-nav").css("display", "none");
});

$("#show-type-all").click(function (){
    $("#all-lines-area").css("display", "block");
    $("#show-type-nav").css("display", "none");
});

$(".back-to-nav").click(function(){
    $("#law-show-area").css("display", "none");
    $("#all-lines-area").css("display", "none");
    $("#show-type-nav").css("display", "block");
});

$(".back-to-search").click(function(){
    $("#show-type-nav").css("display", "none");
    $("#law-search-area").css("display", "flex");
    $("#law-search-area").css("display", "flex");
    $("#law-search-area").css("height", "100vh");
});


$(function() {
    // 使用事件委託，將事件處理程序附加到父元素上
    $("#nav-chapter").on("click", ".chapter-li", async function () {
        $("#all-chapters-ul").fadeOut(1000, function() {
            $(this).remove();
        });

        const buttonText = $(this).text();
        load_all_lines(buttonText);
    });
});

function load_all_search_chapters(chapter) {
    $.ajax({
        url: `http://localhost:9090/search/${chapter}`,
        method: 'GET',
        success: function(response) {
            // 將回應內容加入到 #all-lines
            $("#chapter-ul").append(response);

            $("<style>")
                .prop("type", "text/css")
                .html(`
                .chapter-ul-2 {
                    margin-left: 0; /* 移除縮排 */
                    position: relative; /* 保持在文檔流中 */
                    display: none; /* 預設隱藏 */
                    transform: scaleY(0); /* 初始狀態為縮放為 0，隱藏 */
                    transform-origin: top; /* 縮放的基準點設定為頂部 */
                    transition: transform 0.3s ease-out;
                }
                
                
                li li{
                    float:none;
                    border-top:1px solid #7F9492;
                }
                
                
                .chapter-ul-1 {
                    position: relative; /* 成為子層的參考點 */
                }
                
                /* 展開子層 */
                .chapter-li-1:hover > .chapter-ul-2 {
                    display: block; /* 顯示子層 */
                    transform: scaleY(1); /* 展開到正常大小 */
                }
                `)
                .appendTo("head");
        },
        error: function(xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}




function load_all_chapters() {
    $.ajax({
        url: 'http://localhost:9090/all_chapters',
        method: 'GET',
        success: function(response) {
            // 將回應內容加入到 #all-lines
            $("#nav-chapter").append(response);
        },
        error: function(xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}

load_all_chapters()


/*選單隨畫面卷動*/
$(function (){
    $(window).scroll(function (){
        $("#nav-chapter").stop().animate({"top": $(window).scrollTop()+100},
            500);
    });
});


//下拉式選單
$(document).on('mouseenter', '.chapter-ul-1 > li', function() {
    // 關閉同層其他的 ul
    $(this).siblings().children("ul").stop().slideUp(100);
    // 展開當前 ul
    $(this).children("ul").stop().slideDown(100);
});

$(document).on('mouseleave', '.chapter-ul-1 > li', function() {
    // 隱藏當前 ul
    $(this).children("ul").stop().slideUp(100);
});



$(document).on('click', '.chapter-li-1 > a', function(event) {
        event.stopPropagation(); // 停止事件冒泡
        const chapter1 = $("#chapter_is_law_show").text();
        const chapter2_html = $(this).html();
        const chapter2 = chapter2_html.replace(/<[^>]*>/g, '');
        $.ajax({
            url: `http://localhost:9090/lines_by_chapter/${chapter1}/1/${chapter2}`,
            method: 'GET',
            success: function(response) {
                // 將回應內容加入到 #all-lines
                $("#ttt").html(response);
            },
            error: function(xhr, status, error) {
                console.log("Error: " + error);
            }
        });
        $("#chapter-ul").css("display", "block");
});

$(document).on('click', '.chapter-li-2', function(event) {
    event.stopPropagation(); // 停止事件冒泡
    $("#ttt").html($(this).html());
});

