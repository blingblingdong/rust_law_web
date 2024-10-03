// js/Person.js
export class File {
    constructor(id, content, css, user_name, directory) {
        this.id = id;
        this.content = content;
        this.css = css;
        this.user_name = user_name;
        this.directory = directory;
    }

    greet() {
        console.log(`Hello, ${this.id}`);
    }

    // 工廠方法根據不同 API 回應格式創建 File 物件
    static from_api_v1(api_data) {
        return new File(api_data.file_id, api_data.file_content, api_data.file_css, api_data.file_user, api_data.file_directory);
    }

    static from_api_v2(api_data) {
        return new File(api_data.id, api_data.content, api_data.css, api_data.user_name, api_data.directory);
    }
}
