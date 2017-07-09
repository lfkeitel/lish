package history

import "sync"

type History struct {
	items []string
	mutex sync.Mutex
}

func New() *History {
	return &History{
		items: make([]string, 0, 20),
	}
}

func NewFromFile(path string) *History {
	// TODO Actually load history from a file
	return New()
}

func (h *History) SaveToFile(path string) error {
	// TODO Actually write out to a file
	return nil
}

func (h *History) Len() int {
	h.mutex.Lock()
	defer h.mutex.Unlock()
	return len(h.items)
}

func (h *History) Get(index int) string {
	h.mutex.Lock()
	defer h.mutex.Unlock()
	if index < 0 || index > len(h.items) {
		return ""
	}
	return h.items[index]
}

func (h *History) lastItem() string {
	if len(h.items) == 0 {
		return ""
	}
	return h.items[len(h.items)-1]
}

func (h *History) Add(item string) {
	h.mutex.Lock()
	defer h.mutex.Unlock()
	if item != h.lastItem() {
		h.items = append(h.items, item)
	}
}
