//! Integration test: SVG path parser.

use animato::{CompoundPath, PathCommand, PathEvaluate, SvgPathError, SvgPathParser};

#[test]
fn parses_absolute_and_relative_commands() {
    let commands = SvgPathParser::try_parse("M0 0 l10 0 h10 v10 q10 0 10 10 z").unwrap();

    assert_eq!(commands[0], PathCommand::MoveTo([0.0, 0.0]));
    assert_eq!(commands[1], PathCommand::LineTo([10.0, 0.0]));
    assert_eq!(commands[2], PathCommand::LineTo([20.0, 0.0]));
    assert_eq!(commands[3], PathCommand::LineTo([20.0, 10.0]));
    assert_eq!(
        commands[4],
        PathCommand::QuadTo {
            control: [30.0, 10.0],
            end: [30.0, 20.0],
        }
    );
    assert_eq!(commands[5], PathCommand::ClosePath);
}

#[test]
fn parses_cubic_and_arc_commands() {
    let commands = SvgPathParser::try_parse("M0 0 C10 0 10 20 20 20 A5 5 0 01 30 20").unwrap();

    assert_eq!(commands.len(), 3);
    assert_eq!(
        commands[2],
        PathCommand::ArcTo {
            radii: [5.0, 5.0],
            x_axis_rotation: 0.0,
            large_arc: false,
            sweep: true,
            end: [30.0, 20.0],
        }
    );
}

#[test]
fn parser_handles_adjacent_number_boundaries() {
    let commands = SvgPathParser::try_parse("M10-20L30-40").unwrap();

    assert_eq!(commands[0], PathCommand::MoveTo([10.0, -20.0]));
    assert_eq!(commands[1], PathCommand::LineTo([30.0, -40.0]));
}

#[test]
fn unsupported_command_returns_error_and_permissive_parse_is_empty() {
    let err = SvgPathParser::try_parse("M0 0 S 1 2 3 4").unwrap_err();

    assert!(matches!(err, SvgPathError::UnsupportedCommand { .. }));
    assert!(SvgPathParser::parse("M0 0 S 1 2 3 4").is_empty());
}

#[test]
fn compound_path_from_svg_evaluates() {
    let path = CompoundPath::try_from_svg("M0 0 L100 0 A50 50 0 0 1 200 0 Z").unwrap();

    assert!(path.len() >= 3);
    assert_eq!(path.position(0.0), [0.0, 0.0]);
    assert_eq!(path.position(1.0), [0.0, 0.0]);
    assert!(path.arc_length() > 200.0);
}
