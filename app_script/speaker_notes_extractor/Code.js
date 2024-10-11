function speakerNotes() {
    const presentation = SlidesApp.getActivePresentation();
    const doc = DocumentApp.openById("1xF9mFcSr-e5-DMOo2NPWbU2QzZugeSZ3kNAxQGgSIMQ");
    const doc_body = doc.getBody();
    doc_body.clear();
    const slides = presentation.getSlides();
    slides.forEach((slide,index)=>{
        const note = slide.getNotesPage().getSpeakerNotesShape().getText().asString();
        doc_body.appendParagraph(`#${index+1}\n${note}`);
        doc_body.appendPageBreak();
    });
}
