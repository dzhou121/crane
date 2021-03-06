use std::sync::Arc;

use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, Size, Widget, WidgetExt, WidgetId, WidgetPod,
};

use crate::{
    buffer::{BufferId, BufferNew, BufferState},
    command::{LapceUICommand, LAPCE_UI_COMMAND},
    data::{LapceEditorLens, LapceMainSplitData, LapceTabData},
    editor::LapceEditorView,
    split::LapceSplitNew,
};

pub struct LapceTabNew {
    id: WidgetId,
    main_split: WidgetPod<LapceTabData, Box<dyn Widget<LapceTabData>>>,
}

impl LapceTabNew {
    pub fn new(data: &LapceTabData) -> Self {
        let editor = data.main_split.editors.iter().next().unwrap().1;
        let main_split = LapceSplitNew::new(*data.main_split.split_id)
            .with_flex_child(
                LapceEditorView::new(editor.view_id, editor.editor_id)
                    .lens(LapceEditorLens(editor.view_id))
                    .boxed(),
                1.0,
            );
        Self {
            id: data.id,
            main_split: WidgetPod::new(main_split.boxed()),
        }
    }
}

impl Widget<LapceTabData> for LapceTabNew {
    fn id(&self) -> Option<WidgetId> {
        Some(self.id)
    }

    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut LapceTabData,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                for (_, buffer) in data.main_split.buffers.iter() {
                    if !buffer.loaded {
                        buffer.retrieve_file(
                            data.proxy.clone(),
                            data.id,
                            ctx.get_external_handle(),
                        );
                    }
                }
            }
            Event::Command(cmd) if cmd.is(LAPCE_UI_COMMAND) => {
                let command = cmd.get_unchecked(LAPCE_UI_COMMAND);
                match command {
                    LapceUICommand::LoadBuffer { id, content } => {
                        let buffer = data.main_split.buffers.get_mut(id).unwrap();
                        Arc::make_mut(buffer).load_content(content);
                        data.main_split.notify_update_text_layouts(ctx, id);
                        ctx.set_handled();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        self.main_split.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &LapceTabData,
        env: &Env,
    ) {
        self.main_split.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &LapceTabData,
        data: &LapceTabData,
        env: &Env,
    ) {
        self.main_split.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &LapceTabData,
        env: &Env,
    ) -> Size {
        self.main_split.layout(ctx, bc, data, env);
        self.main_split.set_origin(ctx, data, env, Point::ZERO);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &LapceTabData, env: &Env) {
        self.main_split.paint(ctx, data, env);
    }
}
