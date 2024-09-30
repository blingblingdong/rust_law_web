// otherFile.js
const config = {
    apiUrl: "http://localhost:9090"
};

const test_user_name = "test_user";

document.getElementById('search-law-form').addEventListener('submit', async (event) => {
    event.preventDefault();
    const chapter = $("#chapter").val();
    const num = $("#num").val();
    const user_name = "test_user";
    const directory = "second_folder";
    let id = user_name + "-" + directory + "-" + chapter + "-" + num;

    const response = await fetch(`${config.apiUrl}/questions/${chapter}/${num}`);
    const tableHtml = await response.text();

    const tableContainer = document.getElementById('result-area');
    tableContainer.innerHTML = tableHtml;// 清空表格


    // 清除表單
    event.target.reset();// 重新載入問題列表

});


$(function () {
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
    document.getElementById('record-area').style.display = 'block';
    document.getElementById('test-area').style.display = 'none';
});

document.getElementById('test-btn').addEventListener('click', () => {
    document.getElementById('search-area').style.display = 'none';
    document.getElementById('record-area').style.display = 'none';
    document.getElementById('test-area').style.display = 'flex';
});

$(function () {
    // 使用事件委託，將事件處理程序附加到父元素上
    $("#record_table").on("click", ".record-button", async function () {
        $("#show-words").remove();
        $("#show-words").fadeOut(1000);
        const buttonText = $(this).text();
        const [element1, element2] = buttonText.split('-');
        const response = await fetch(`${config.apiUrl}/questions/${element1}/${element2}`);
        const tableHtml = await response.text();
        $("#record_show").append(`<div id='show-words' style='display: none;'>${tableHtml}</div>`);
        $("#show-words").fadeIn(1000);
    });
});

function load_all_lines(chapter) {
    $.ajax({
        url: `${config.apiUrl}/questions/all_lines/${chapter}`,
        method: 'GET',
        success: function (response) {
            // 將回應內容加入到 #all-lines
            $("#all-lines").html(`${response}`);

            // 動態加入 CSS 樣式
            $("<style>")
                .prop("type", "text/css")
                .html(`
                    .law-content {
                        margin-bottom: 20px;
                        padding: 5px 0;
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
        },
        error: function (xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}


$("#law-search-area-form").submit(function (event) {
    event.preventDefault();
    const chapter = $("#search-chapter").val();
    $("#law-search-area").css("display", "none");
    $("#show-type-nav").css("display", "flex");
    $(".chapter_is").html(chapter);
    event.target.reset();
});


$("#show-type-chapter").click(function (event) {
    event.stopPropagation();
    $("#law-show-area").css("display", "block");
    load_all_search_chapters($(".chapter_is").html());
    $("#show-type-nav").css("display", "none");
});

$("#show-type-all").click(function (event) {
    event.stopPropagation();
    $("#all-lines-area").css("display", "block");
    load_all_lines($(".chapter_is").html());
    $("#show-type-nav").css("display", "none");
});

$(".back-to-nav").click(function () {
    $("#law-show-area").css("display", "none");
    $("#all-lines-area").css("display", "none");
    $("#show-type-nav").css("display", "flex");
});

$(".back-to-search").click(function () {
    $("#show-type-nav").css("display", "none");
    $("#law-search-area").css("display", "flex");
});


function load_all_search_chapters(chapter) {
    $.ajax({
        url: `http://localhost:9090/search/${chapter}`,
        method: 'GET',
        success: function (response) {
            // 將回應內容加入到 #all-lines
            $("#chapter-ul").html(response);

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
        error: function (xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}


function load_all_dir() {
    $.ajax({
        url: `${config.apiUrl}/all_dir/test_user`,
        method: 'GET',
        success: function (response) {
            // 將回應內容加入到 #all-lines
            $("#dir").html(response);
            $("#dir").append("<li class='add-dir'>新增資料</li>");
            $(".add-dir").css("padding", "10px 10px");
            $(".the-dir").css("padding", "10px 10px");
            $(".the-dir").css("font-size", "20px");
        },
        error: function (xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}


function load_all_chapters() {
    $.ajax({
        url: `${config.apiUrl}/all_chapters`,
        method: 'GET',
        success: function (response) {
            // 將回應內容加入到 #all-lines
            $("#nav-chapter").html(response);
        },
        error: function (xhr, status, error) {
            console.log("Error: " + error);
        }
    });
}

load_all_chapters()


/*選單隨畫面卷動*/
$(function () {
    $(window).scroll(function () {
        $("#nav-chapter").stop().animate({"top": $(window).scrollTop() + 100},
            500);
    });
});


//下拉式選單
$(document).on('mouseenter', '.chapter-ul-1 > li', function () {
    // 關閉同層其他的 ul
    $(this).siblings().children("ul").stop().slideUp(100);
    // 展開當前 ul
    $(this).children("ul").stop().slideDown(100);
});

$(document).on('mouseleave', '.chapter-ul-1 > li', function () {
    // 隱藏當前 ul
    $(this).children("ul").stop().slideUp(100);
});


$(document).on('click', '.chapter-li-1 > a', function (event) {
    event.stopPropagation(); // 停止事件冒泡
    const chapter1 = $("#chapter_is_law_show").text();
    const chapter2_html = $(this).html();
    const chapter2 = chapter2_html.replace(/<[^>]*>/g, '');
    $.ajax({
        url: `${config.apiUrl}/lines_by_chapter/${chapter1}/1/${chapter2}`,
        method: 'GET',
        success: function (response) {
            // 將回應內容加入到 #all-lines
            $("#ttt").html(response);
        },
        error: function (xhr, status, error) {
            console.log("Error: " + error);
        }
    });
    $("#chapter-ul").css("display", "block");
});


/*關於records*/
$(document).on('click', '.the-dir', function () {// 停止事件冒泡
    loadQuestions($(this).text());
    $("#in_folder").css("display", "block");
    $("#dir").css("display", "none");
    $(".record-footer").css("display", "flex");
    $("#folder-name").html($(this).text());
    $(".header-container").addClass("hideproperty");
});

$(document).on('click', '#delete-dir', function () {// 停止事件冒泡
    let dir = $("#folder-name").text();
    let answer = confirm(`確認刪除資料夾${dir}?`);
    if (answer) {
        $.ajax({
            url: `${config.apiUrl}/delete_dir_by_name/${dir}`,
            method: 'DELETE',
            success: function (response) {
                alert("刪除成功")
                $("#dir").css("display", "grid");
                $("#record_table").css("display", "none");
                $(".record-footer").css("display", "none")
                load_all_dir();
            },
            error: function (xhr, status, error) {
                alert("刪除失敗")
            }
        });
    }

});

async function loadQuestions(directory) {
    const response = await fetch(`${config.apiUrl}/records_to_laws/test_user/${directory}`);
    const tableHtml = await response.text();

    const tableContainer = document.getElementById('record_table');
    tableContainer.innerHTML = tableHtml;// 清空表格

}

//編輯器與卡片區的轉換
$("#record-card-btn").click(function () {
    $("#record_table").css("display", "block");
    $(".record-footer").css("display", "flex");
    $("#record-editor").css("display", "none");
});

$("#record-editor-btn").click(async function () {
    $("#record_table").css("display", "none");
    $("#record-editor").css("display", "block");
    $(".record-footer").css("display", "flex");
    let id = test_user_name + "-" + $("#folder-name").text();
    $.ajax({
        url: `${config.apiUrl}/file_html/${id}`,
        method: 'GET',
        success: function (response) {
            $("#word-area").html(response);
            let number =  $("#word-area").text().length;
            $("#text-number").html(number);
        },
        error: function (xhr, status, error) {
            alert("失敗");
            console.log("Error: " + error);
        }
    });

});

$("#back_to_folder").click(function () {
    $("#dir").css("display", "grid");
    $("#in_folder").css("display", "none");
    $(".record-footer").css("display", "none");
    $(".header-container").removeClass("hideproperty");
    load_all_dir();
});

load_all_dir();

$(document).on('click', '.add-dir', function () {
    const popHTML = `
                <div class="popup" id="popup2">
                <div class="popup-content">
                <div class="popup-header">
                    <h3>加入資料夾</h3>
                    <span class="close-btn" onclick="hidePopup2()">X</span>
                </div>
                <div class="popup-body">
                    <form id="create-dir-form" style="display: flex; flex-direction: column;">
                        <input type="text" id="dir-name" placeholder="目錄名稱" required>
                        <button type="submit">創建</button>
                    </form>
                </div>
                </div>
                </div>`
    document.body.insertAdjacentHTML('beforeend', popHTML);

    // 顯示彈出視窗
    document.getElementById('popup2').style.display = 'flex';

    $(document).on('submit', '#create-dir-form', async function (event) {
        event.preventDefault();
        const dir = $("#dir-name").val();
        add_to_dir("創建", "創建", "test_user", dir);
        load_all_dir();
        event.target.reset();
    });
});


$(document).on('click', '.add-law', function () {
    const buttonId = $(this).attr('id');
    // 顯示彈出視窗
    showPopup();

    // 將按鈕的 id 顯示在 #pop-law 裡面
    $("#pop-law").html(`${buttonId}`);
});

function showPopup() {
    // 建立彈出視窗的 HTML
    const popupHTML = `
        <div class="popup" id="popup">
            <div class="popup-content">
                <div id="pop-law"></div>
                <div class="popup-header">
                    <h3>加入資料夾</h3>
                    <span class="close-btn" onclick="hidePopup()">X</span>
                </div>
                <div class="popup-body" id="popup-options">
                    <!-- 選項將會在這裡動態插入 -->
                </div>
                <button onclick="addFolder()">新增資料夾</button>
                <form id="add_dir_form" style="display: none;">
                    <input type="text" id="pop-dir-name" placeholder="目錄名稱" required>
                    <button type="submit">創建</button>
                </form>
                <div class="popup-footer">
                    <button onclick="confirmSelection()">確定</button>
                </div>
            </div>
        </div>
    `;

    // 插入彈出視窗到 body
    document.body.insertAdjacentHTML('beforeend', popupHTML);

    // 顯示彈出視窗
    document.getElementById('popup').style.display = 'flex';

    // 發送 AJAX 請求並將結果插入到 .popup-body
    $.ajax({
        url: `${config.apiUrl}/dir_for_pop/test_user`,
        method: 'GET',
        success: function (response) {
            // 將回應內容加入到 #popup-options
            $("#popup-options").html(response);
        },
        error: function (xhr, status, error) {
            console.log("Error: " + error);
        }
    });

    $(document).on('submit', '#add_dir_form', function (event) {
        event.preventDefault(); // 阻止表單提交後刷新頁面
        const dirName = $("#pop-dir-name").val();

        if (dirName) {
            add_to_dir("創建", "創建", "test_user", dirName);
            alert(`創建的資料夾名稱: ${dirName}`);
            $("#add_dir_form").css("display", "none");
            $.ajax({
                url: `${config.apiUrl}/dir_for_pop/test_user`,
                method: 'GET',
                success: function (response) {
                    // 將回應內容加入到 #popup-options
                    $("#popup-options").html(response);
                },
                error: function (xhr, status, error) {
                    console.log("Error: " + error);
                }
            });
        } else {
            alert("請輸入資料夾名稱");
        }
    });
}


function hidePopup() {
    // 移除彈出視窗元素
    const popup = document.getElementById('popup');
    if (popup) {
        popup.remove();
    }
}

function hidePopup2() {
    // 移除彈出視窗元素
    const popup = document.getElementById('popup2');
    if (popup) {
        popup.remove();
    }
}

function addFolder() {
    $("#add_dir_form").css("display", "flex");
}


function confirmSelection() {
    // 獲取所有被打勾的 checkbox
    const checkedOptions = document.querySelectorAll('.popup-body input[type="checkbox"]:checked');

    // 用來儲存選中的資料夾名稱
    const selectedFolders = [];

    const [element1, chapter, num] = $("#pop-law").text().split('-');

    // 遍歷每個被打勾的 checkbox，並獲取其相對應的 label 文字
    checkedOptions.forEach(option => {
        // 獲取對應的 label 文字
        const label = option.nextElementSibling; // label 緊跟在 checkbox 後面
        if (label) {
            selectedFolders.push(label.innerText);
        }
    });

    // 顯示選中的資料夾名稱
    if (selectedFolders.length > 0) {
        selectedFolders.forEach(dir => {
            add_to_dir(chapter, num, "test_user", dir);
        });
        alert('成功新增');
    } else {
        alert('沒有選中的資料夾');
    }
    hidePopup();
}

async function add_to_dir(chapter, num, user_name, directory) {
    let id = user_name + "-" + directory + "-" + chapter + "-" + num;
    const question = {id: id, chapter: chapter, num: num, user_name: user_name, directory: directory, note: "新增筆記"};
    const url = `${config.apiUrl}/questions/${chapter}/${num}`;

    try {
        if (chapter != "創建") {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error('無此條目');
            }
        } else {
            add_file(test_user_name, directory, "# 請用markdown寫入筆記!", "no");
        }
        // 如果 response 有回傳數據且你需要使用的話
        await fetch(`${config.apiUrl}/questions`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(question),
        });
        alert("成功加入");
    } catch (error) {
        alert(error.message);
        console.log("Error: " + error);
    }
}


/*顯示資料夾內筆記*/
$(document).on('click', '.toggle-note-law', function () {
    const [element1, element2, chapter, num] = $(this).attr('id').split('-');
    let law_note = "#" + "card-law-note" + "-" + chapter + "-" + num;
    $(law_note).css("display", "flex");
});


//隱藏筆記部分
$(document).on('click', '.note-hide-btn', function () {
    const [element1, element2, element3, chapter, num] = $(this).attr('id').split('-');
    let law_note = "#" + "card-law-note" + "-" + chapter + "-" + num;
    $(law_note).css("display", "none");
});

//編輯筆記
$(document).on('click', '.note-edit-btn', function () {
    const [element1, element2, element3, chapter, num] = $(this).attr('id').split('-');
    let note_area_id = "#law-note-area-" + chapter + "-" + num;
    let original_note = $(note_area_id).html();
    const formHTML = `
        <form class="law-note-form" id="law-note-form-${chapter}-${num}">
            <textarea class="note-form-text" id="note-form-text-${chapter}-${num}">${original_note}</textarea>
            <button class="note-form-btn" type="submit"></button>
        </form>
    `;
    $(note_area_id).html(formHTML);
});

// 將提交事件處理器移出點擊事件處理器外部
$(document).on('submit', '.law-note-form', async function (event) {
    event.preventDefault(); // 阻止表單提交後刷新頁面
    let formId = $(this).attr('id');
    let textareaId = `#note-form-text-${formId.split('-')[3]}-${formId.split('-')[4]}`;
    let new_note = $(textareaId).val();
    alert("lll");
    update_note("test_user", $("#folder-name").text(), formId.split('-')[3],  formId.split('-')[4], new_note);
    let note_area_id = "#law-note-area-" + formId.split('-')[3] + "-" + formId.split('-')[4];
    $(note_area_id).html(new_note);
});

async function update_note(user, dir, chapter, num, note) {
    let id = user + "-" + dir + "-" + chapter + "-" + num;
    $.ajax({
        url: `${config.apiUrl}/update_note/${id}`,
        method: 'PUT',
        contentType: 'application/json', // 确保发送 JSON 格式
        data: JSON.stringify({ note: note }), // 将 note 包装在 JSON 对象中
        success: function (response) {
            alert("成功更新");
        },
        error: function (xhr, status, error) {
            alert("更新失败");
            console.log("Error: " + error);
        }
    });
}




$(document).on('submit', '.card-add-form', async function (event) {
    event.preventDefault(); // 阻止表單提交後刷新頁面
    let folder = $("#folder-name").text();
    let chapter = $("#card-form-chapter").val();
    let num = $("#card-form-num").val();
    add_to_dir(chapter, num, "test_user", folder);
    loadQuestions(folder);
});




document.addEventListener('DOMContentLoaded', function () {
    // 获取按钮和 textarea 的引用
    const addButton = document.getElementById('add-card');
    const markdownTextarea = document.getElementById('markdown');

    // 为按钮添加点击事件处理器
    addButton.addEventListener('click', function () {
        // 在当前 textarea 内容后追加 "card" 文字
        markdownTextarea.value += "card";  // 你可以选择添加空格或换行符来格式化
    });
});

$("#add-bold").click(function () {
    $("#markdown").append("**粗體字**");
});

$("#add-picture").click(function () {
    $("#markdown").append("![](照片網址)");
});

$("#add-css").click(function () {
    const popupHTML = `
        <div class="popup" id="popup-css">
            <div class="popup-content">
                <div class="popup-header">
                    <h3>加入css</h3>
                    <span class="close-btn" onclick="hidePopup_css()">X</span>
                </div>
                <div class="popup-body">
                   <form id="add_css_form">
                    <input type="text" id="css-name" placeholder="css名稱" required>
                    <input type="text" id="css-u1" placeholder="css內容" required>
                    <input type="text" id="css-u2" placeholder="css內容" required>
                    <button type="submit">創建</button>
                    </form>
                </div>
                <div class="popup-footer">
                    <button onclick="hidePopup_css()">確定</button>
                </div>
            </div>
        </div>
    `;

    // 插入彈出視窗到 body
    document.body.insertAdjacentHTML('beforeend', popupHTML);

    // 顯示彈出視窗
    document.getElementById('popup-css').style.display = 'flex';


    $(document).on('submit', '#add_css_form', function (event) {
        event.preventDefault(); // 阻止表單提交後刷新頁面
        const CssName = $("#css-name").val();
        const CssU1 = $("#css-u1").val();
        const CssU2 = $("#css-u2").val();
        let name = "#preview" + " " + CssName;
        $(name).css(CssU1, CssU2);
    });
});

function hidePopup_css() {
    // 移除彈出視窗元素
    const popup = document.getElementById('popup-css');
    if (popup) {
        popup.remove();
    }
}




document.addEventListener('DOMContentLoaded', function() {
    const colorPicker = document.getElementById('color-picker');
    const colorButton = document.getElementById('add-color');
    const markdownTextarea = document.getElementById('markdown');

    // 當點擊調色盤按鈕時，觸發顏色選擇器
    colorButton.addEventListener('click', function() {
        $("#color-picker").css("display", "flex");
        colorPicker.click();
    });

    // 當顏色選擇器的顏色改變時，更新按鈕顏色並應用於 textarea
    colorPicker.addEventListener('input', function() {
        const color = colorPicker.value;
        colorButton.style.color = color; // 改變調色盤按鈕的顏色
        let span1 = `<span style='color:`
        let span2 = `;'> 有顏色的字</span>`
        let span3 = span1 + color + span2;
        document.getElementById('markdown').value += span3;
        $("#color-picker").css("display", "none");
    });
});

$("#record-viewer-tools-edit").click(function () {
    $("#record-writer").css("display", "flex");
    $("#record-viewer").css("display", "none");
    $(".record-footer").css("display", "none");
    let id = test_user_name + "-" + $("#folder-name").text();
    alert(id);

    $.ajax({
        url: `${config.apiUrl}/file_markdown/${id}`,
        method: 'GET',
        success: function (response) {
            $("#markdown").html(response);
        },
        error: function (xhr, status, error) {
            alert("失敗");
            console.log("Error: " + error);
        }
    });

});


// 處理確認修改筆記
$("#confirm-edit").click(async function () {
    let text = $("#preview").html();

    let id = test_user_name + "-" + $("#folder-name").text();
    let content = $("#markdown").val();
    alert(content);

    //更新筆記內容
    $.ajax({
        url: `${config.apiUrl}/file/${id}`,
        method: 'PUT',
        contentType: 'application/json', // 确保发送 JSON 格式
        data: JSON.stringify({ content: content }), // 将 note 包装在 JSON 对象中
        success: function (response) {
            alert("成功更新");
        },
        error: function (xhr, status, error) {
            alert("更新失败");
            console.log("Error: " + error);
        }
    });

    $("#word-area").html(text);


    // 更新viewer
    $.ajax({
        url: `${config.apiUrl}/file_html/${id}`,
        method: 'GET',
        success: function (response) {
            $("#word-area").html(response);
            let number =  $("#word-area").text().trim().length;
            $("#text-number").html(number);
        },
        error: function (xhr, status, error) {
            alert("失敗");
            console.log("Error: " + error);
        }
    });

    // 更新筆記字數
    let number = $("#preview").text().length;
    $("#text-number").html(number);

    $("#record-writer").css("display", "none");
    $("#record-viewer").css("display", "flex");
    $(".record-footer").css("display", "flex");
});



async function add_file(user_name, directory, content, css) {
    let id = user_name + "-" + directory;
    const file = {id: id, content: content, css: css, user_name: user_name, directory: directory};

    try {
        await fetch(`${config.apiUrl}/file`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(file),
        });
        alert("成功加入");
    } catch (error) {
        alert(error.message);
        console.log("Error: " + error);
    }
}



