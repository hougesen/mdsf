use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_blade_formatter_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--write").arg(snippet_path);
}

#[inline]
fn invoke_blade_formatter(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_blade_formatter_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_blade_formatter(std::process::Command::new("blade-formatter"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_blade_formatter(setup_npm_script("blade-formatter"), snippet_path)
}

#[cfg(test)]
mod test_blade_formatter {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(blade-formatter)]
    fn it_should_format_blade() {
        let input = r#"@extends('frontend.layouts.app')
@section('title') foo
@endsection
@section('content')
<section id="content">
<div class="container mod-users-pd-h">
    <div class="pf-user-header">
    <div></div>
    <p>@lang('users.index')</p>
    </div>
        <div class="pf-users-branch">
            <ul class="pf-users-branch__list">
                @foreach($users as $user)
        <li>
            <img src="{{ asset('img/frontend/icon/branch-arrow.svg') }}" alt="branch_arrow">
            {{ link_to_route("frontend.users.user.show",$users["name"],$users['_id']) }}
        </li>
        @endforeach
      </ul>
      <div class="pf-users-branch__btn">
      @can('create', App\Models\User::class)
            {!! link_to_route("frontend.users.user.create",__('users.create'),[1,2,3],['class' => 'btn']) !!}
            @endcan
        </div>
  </div>
    </div>
</section>
@endsection
@section('footer')
@stop"#;

        let expected_output = r#"@extends('frontend.layouts.app')
@section('title') foo
@endsection
@section('content')
    <section id="content">
        <div class="container mod-users-pd-h">
            <div class="pf-user-header">
                <div></div>
                <p>@lang('users.index')</p>
            </div>
            <div class="pf-users-branch">
                <ul class="pf-users-branch__list">
                    @foreach ($users as $user)
                        <li>
                            <img src="{{ asset('img/frontend/icon/branch-arrow.svg') }}" alt="branch_arrow">
                            {{ link_to_route('frontend.users.user.show', $users['name'], $users['_id']) }}
                        </li>
                    @endforeach
                </ul>
                <div class="pf-users-branch__btn">
                    @can('create', App\Models\User::class)
                        {!! link_to_route('frontend.users.user.create', __('users.create'), [1, 2, 3], ['class' => 'btn']) !!}
                    @endcan
                </div>
            </div>
        </div>
    </section>
@endsection
@section('footer')
@stop
"#;

        let snippet =
            setup_snippet(input, &language_to_ext("sql")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
