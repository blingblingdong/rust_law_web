import {MyButton} from './MyCustomButton.js';


import {
    AccessibilityHelp,
    Autoformat,
    Autosave,
    BalloonToolbar,
    BlockQuote,
    Bold,
    Essentials,
    FullPage,
    GeneralHtmlSupport,
    Heading,
    HtmlComment,
    HtmlEmbed,
    Indent,
    IndentBlock,
    Italic,
    Link,
    Paragraph,
    SelectAll,
    ShowBlocks,
    SourceEditing,
    Table,
    TableCaption,
    TableCellProperties,
    TableColumnResize,
    TableProperties,
    TableToolbar,
    TextTransformation,
    Underline,
    Undo,
    Image,
    ImageInsert,
    ImageBlock,
    ImageInline,
    ImageInsertViaUrl,
    ImageResize,
    ImageStyle,
    ImageToolbar,
    ImageUpload,
    FontBackgroundColor,
    FontColor,
    FontFamily,
    FontSize,
} from 'ckeditor5';

export const editorConfig = {
    toolbar: {
        items: [
            'undo',
            'redo',
            '|',
            {
                label: 'Fonts',
                items: ['sourceEditing', 'showBlocks','bold', 'italic', 'underline','link', 'insertTable', 'blockQuote', 'outdent', 'indent', 'insertImage', 'fontColor', 'fontBackgroundColor']
            },
            'heading',
            '-',
            'fontFamily',
            'mybutton'
        ],
        shouldNotGroupWhenFull: true
    },
    plugins: [
        MyButton,
        AccessibilityHelp,
        FontBackgroundColor,
        FontColor,
        FontFamily,
        FontSize,
        Autoformat,
        Autosave,
        BalloonToolbar,
        BlockQuote,
        Bold,
        Essentials,
        FullPage,
        GeneralHtmlSupport,
        Heading,
        HtmlComment,
        HtmlEmbed,
        Indent,
        IndentBlock,
        Italic,
        Link,
        Paragraph,
        SelectAll,
        ShowBlocks,
        SourceEditing,
        Table,
        TableCaption,
        TableCellProperties,
        TableColumnResize,
        TableProperties,
        TableToolbar,
        TextTransformation,
        Underline,
        Image,
        ImageInsert,
        Undo,
        ImageBlock,
        ImageInline,
        ImageInsertViaUrl,
        ImageResize,
        ImageStyle,
        ImageToolbar,
        ImageUpload,
    ],
    balloonToolbar: ['bold', 'italic', '|', 'link', 'fontFamily', 'mybutton'],
    heading: {
        options: [
            {
                model: 'paragraph',
                title: 'Paragraph',
                class: 'ck-heading_paragraph'
            },
            {
                model: 'heading1',
                view: 'h1',
                title: 'Heading 1',
                class: 'ck-heading_heading1'
            },
            {
                model: 'heading2',
                view: 'h2',
                title: 'Heading 2',
                class: 'ck-heading_heading2'
            },
            {
                model: 'heading3',
                view: 'h3',
                title: 'Heading 3',
                class: 'ck-heading_heading3'
            },
            {
                model: 'heading4',
                view: 'h4',
                title: 'Heading 4',
                class: 'ck-heading_heading4'
            },
            {
                model: 'heading5',
                view: 'h5',
                title: 'Heading 5',
                class: 'ck-heading_heading5'
            },
            {
                model: 'heading6',
                view: 'h6',
                title: 'Heading 6',
                class: 'ck-heading_heading6'
            }
        ]
    },
    htmlSupport: {
        allow: [
            {
                name: /.*/, // 匹配所有元素
                attributes: ['class', 'id'],
            }
        ]
    },
    image: {
        toolbar: ['imageTextAlternative', '|', 'imageStyle:inline', 'imageStyle:wrapText', 'imageStyle:breakText', '|', 'resizeImage']
    },
    initialData:   "new",
    link: {
        addTargetToExternalLinks: true,
        defaultProtocol: 'https://',
        decorators: {
            toggleDownloadable: {
                mode: 'manual',
                label: 'Downloadable',
                attributes: {
                    download: 'file'
                }
            }
        }
    },
    placeholder: 'Type or paste your content here!',
    table: {
        contentToolbar: ['tableColumn', 'tableRow', 'mergeTableCells', 'tableProperties', 'tableCellProperties']
    }
};


