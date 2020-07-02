const { setup } = require('shellshot');

setup();

function cargo_e2e(name, custom) {
    return async () => {
        await expect.command(`cargo build --example end2end_${name}`)
            .forExitCode(exp => exp.toBe(0));
        await expect.command(
            `tmux new-session -x 80 -y 24 -d 'sh -c "TERM=xterm-256color ../target/debug/examples/end2end_${name}"' \; set status off && sleep 0.05`
        ).forExitCode(exp => exp.toBe(0));

        if (!!custom) {
            await custom();
        }

        await expect.command('tmux capture-pane -J -p -t %0')
            .forStdout(exp => exp.toMatchSnapshot());
        await expect.command('tmux kill-server')
            .forExitCode(exp => exp.toBe(0));
    };
}

it('aligns the child view at top-left',   cargo_e2e('top_left'));
it('aligns the child view at top-center', cargo_e2e('top_center'));
it('aligns the child view at top-right',  cargo_e2e('top_right'));
it('aligns the child view at center-left',  cargo_e2e('center_left'));
it('aligns the child view at center',       cargo_e2e('center'));
it('aligns the child view at center-right', cargo_e2e('center_right'));
it('aligns the child view at bottom-left',   cargo_e2e('bottom_left'));
it('aligns the child view at bottom-center', cargo_e2e('bottom_center'));
it('aligns the child view at bottom-right',  cargo_e2e('bottom_right'));

it('aligns the child view at top-left via composition',   cargo_e2e('align_top_left'));
it('aligns the child view at top-center via composition', cargo_e2e('align_top_center'));
it('aligns the child view at top-right via composition',  cargo_e2e('align_top_right'));
it('aligns the child view at center-left via composition',  cargo_e2e('align_center_left'));
it('aligns the child view at center via composition',       cargo_e2e('align_center'));
it('aligns the child view at center-right via composition', cargo_e2e('align_center_right'));
it('aligns the child view at bottom-left via composition',   cargo_e2e('align_bottom_left'));
it('aligns the child view at bottom-center via composition', cargo_e2e('align_bottom_center'));
it('aligns the child view at bottom-right via composition',  cargo_e2e('align_bottom_right'));
