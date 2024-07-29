#[derive(PartialEq, Debug)]
enum AnimationState {
    Normal,
    Reverse,
}

#[derive(Debug)]
struct RevAnimation {
    state: AnimationState,
    normal: animation::Animation,
    reverse: animation::Animation,
}

impl RevAnimation {
    fn new(textures: Vec<Texture>, durations: Vec<Duration>) -> tetra::Result<RevAnimation> {
        let mut reversed_texes = textures.clone().to_vec();
        reversed_texes.reverse();
        let mut reversed_durs = durations.clone().to_vec();
        reversed_durs.reverse();
        Ok(RevAnimation {
            state: AnimationState::Normal,
            normal: animation::Animation::new(
                textures.clone(),
                durations,

            ),
            reverse: animation::Animation::new(
                reversed_texes,
                reversed_durs,
            ),
        })
    }

    fn draw<P>(&self, ctx: &mut Context, params: P)
    where
        P: Into<DrawParams>,
    {
        self.current().draw(ctx, params)
    }

    fn current(&self) -> &animation::Animation {
        match self.state {
            AnimationState::Normal => &self.normal,
            AnimationState::Reverse => &self.reverse,
        }
    }

    fn current_mut(&mut self) -> &mut animation::Animation {
        match self.state {
            AnimationState::Normal => &mut self.normal,
            AnimationState::Reverse => &mut self.reverse,
        }
    }

    fn advance(&mut self, ctx: &Context) {
        self.current_mut().advance(ctx);
    }

    fn set_state(&mut self, state: AnimationState) {
        if self.state != state {
            self.state = state;
            self.current_mut().restart();
        }
    }
}