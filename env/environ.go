package env

import "strings"

type Environment map[string]string

func New() Environment {
	return Environment(make(map[string]string))
}

func FromSlice(s []string) Environment {
	env := make(Environment, len(s))

	for _, v := range s {
		parts := strings.SplitN(v, "=", 2)
		env[parts[0]] = parts[1]
	}

	return env
}

func (e Environment) ToSlice() []string {
	env := make([]string, len(e))
	i := 0
	for k, v := range e {
		env[i] = k + "=" + v
		i++
	}
	return env
}

func (e Environment) IsSet(key string) bool {
	_, exists := e[key]
	return exists
}

func (e Environment) Set(key, value string) {
	e[key] = value
}

func (e Environment) Get(key string) string {
	return e[key]
}

func (e Environment) GetDefault(key, def string) string {
	val := e[key]
	if val == "" {
		return def
	}
	return val
}

func (e Environment) GetOK(key string) (string, bool) {
	val, ok := e[key]
	return val, ok
}
