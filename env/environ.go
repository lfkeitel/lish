package env

import "strings"

type Environment struct {
	vals   map[string]string
	parent *Environment
}

func New() *Environment {
	return &Environment{
		vals:   make(map[string]string),
		parent: nil,
	}
}

func FromSlice(s []string) *Environment {
	env := New()

	for _, v := range s {
		parts := strings.SplitN(v, "=", 2)
		env.vals[parts[0]] = parts[1]
	}

	return env
}

func (e *Environment) ToSlice() []string {
	env := make([]string, len(e.vals))
	i := 0
	for k, v := range e.vals {
		env[i] = k + "=" + v
		i++
	}
	return env
}

func (e *Environment) IsSet(key string) bool {
	_, exists := e.vals[key]
	return exists
}

func (e *Environment) Set(key, value string) {
	e.vals[key] = value
}

func (e *Environment) Get(key string) string {
	return e.vals[key]
}

func (e *Environment) GetDefault(key, def string) string {
	val := e.vals[key]
	if val == "" {
		return def
	}
	return val
}

func (e *Environment) GetOK(key string) (string, bool) {
	val, ok := e.vals[key]
	return val, ok
}
