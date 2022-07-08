package logger

func Info(template string, args ...interface{}) {
	Logger.Infof(template, args...)
}

func Error(template string, args ...interface{}) {
	Logger.Errorf(template, args...)
}
