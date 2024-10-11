function speakerNotes() {
    const presentation = SlidesApp.getActivePresentation();
    const slides = presentation.getSlides();
    let allNotes = "";
    slides.forEach((slide,index)=>{
        const note = slide.getNotesPage().getSpeakerNotesShape().getText().asString();
        allNotes += `#${index+1}\n`;
        allNotes += note;
    });
    const doc = DocumentApp.openById("1xF9mFcSr-e5-DMOo2NPWbU2QzZugeSZ3kNAxQGgSIMQ");
    doc.getBody().setText(allNotes);
}
