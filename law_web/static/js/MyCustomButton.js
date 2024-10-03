// myCustomButton.js
import { Plugin, ButtonView } from 'ckeditor5'


export class MyButton extends Plugin {
    init() {
        const editor = this.editor;

        editor.ui.componentFactory.add( 'mybutton', () => {
            // The button will be an instance of ButtonView.
            const button = new ButtonView();

            button.set( {
                label: 'Timestamp',
                withText: true
            } );

            // Execute a callback function when the button is clicked
            button.on( 'execute', () => {
                const now = new Date();

                // Change the model using the model writer
                editor.model.change( writer => {
                    // Insert the text at the user's current position
                    editor.model.insertContent( writer.createText( now.toString() ) );
                } );
            } );

            return button;
        } );
    }
}



