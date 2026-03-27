package org.example;

import java.util.AbstractCollection;
import java.util.Iterator;
import java.util.Map;

public class BagImpl<T> extends AbstractCollection<T> implements Bag<T> {

    Map<T, Integer> elements;

    public BagImpl() {
        // this.elements = ....
    }

    @Override
    public boolean add(T element) {
        if (elements.containsKey(element)) {
            int valor = elements.get(element);
            valor++;
            elements.put(element,valor);
        }else {
            elements.put(element, 1);
        }
        return true;
    }

    @Override
    public int occurrencesOf(T element) {
        if (elements.containsKey(element)) {
            return elements.get(element);
        }
        return 0;
    }

    @Override
    public void removeOccurrence(T element) {
        if (elements.containsKey(element)) {
            int valor = elements.get(element);
            if (valor > 1) {
                elements.put(element, valor - 1);
            } else {
                elements.remove(element);
            }
        }
    }

    @Override
    public void removeAll(T element) {
        elements.remove(element);
    }

    @Override
    public int size() {
        Integer total=0;
        for (Integer value : elements.values()){
            total+= value;
        }
       return total;
    }

    @Override
    public Iterator<T> iterator() {
        return new Iterator<T>() {
            private Iterator<Map.Entry<T, Integer>> entryIterator = elements.entrySet().iterator();
            private Map.Entry<T, Integer> currentEntry;
            private int remainingCount;

            @Override
            public boolean hasNext() {
                return remainingCount > 0 || entryIterator.hasNext();
            }

            @Override
            public T next() {
                if (remainingCount == 0) {
                    currentEntry = entryIterator.next();
                    remainingCount = currentEntry.getValue();
                }
                remainingCount--;
                return currentEntry.getKey();
            }

            @Override
            public void remove() {
                removeOccurrence(currentEntry.getKey());
            }
        };
    }
}
