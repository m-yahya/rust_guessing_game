const gulp = require('gulp');
const watch = require('gulp-watch');
const run = require('gulp-run-command').default;

gulp.task('run', run('cargo run'));

gulp.task('watch', () => {
  gulp.watch('./src/**/*.rs', gulp.series('run'));
});
