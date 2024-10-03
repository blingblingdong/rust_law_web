$(function(){
    $("#j-button").click(function () {
        $(this).html("已點擊");
    });
});

$(document).on('click', '.chapter-li-2 > a', function(event) {
    event.stopPropagation(); // 停止事件冒泡
    const chapter1 = $("#chapter_is_law_show").text();
    const chapter2_html = $(this).html();
    const chapter2 = chapter2_html.replace(/<[^>]*>/g, '');
    $.ajax({
        url: `http://localhost:9090/lines_by_chapter/${chapter1}/2/${chapter2}`,
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


