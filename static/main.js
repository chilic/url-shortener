$(function () {
    $("#url-form").submit(function (e) {
        $.ajax({
            url: "/api/v1/generate",
            method: "POST",
            data: JSON.stringify({ url : $("#basic-url").val() }),
            dataType: "json",
            contentType: "application/json; charset=utf-8",
        }).done(function (data) {
            $(this).addClass("done");
            $("#basic-addon3").text(
                $(location).attr('href') + data
            );
        });
        return false;
    });
});
