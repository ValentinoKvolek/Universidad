package org.example;

import java.util.*;
import java.util.function.Consumer;
import java.util.function.IntFunction;
import java.util.function.Predicate;
import java.util.stream.Stream;

public class EvenNumberSet implements Set<Integer> {
    private Set<Integer> internalSet = new HashSet<>();

    @Override
    public int size() {
        return 0;
    }

    @Override
    public boolean isEmpty() {
        return false;
    }

    @Override
    public boolean contains(Object o) {
        return false;
    }

    @Override
    public Iterator iterator() {
        return null;
    }

    @Override
    public void forEach(Consumer action) {
        Set.super.forEach(action);
    }

    @Override
    public Object[] toArray() {
        return new Object[0];
    }

    @Override
    public Object[] toArray(IntFunction generator) {
        return Set.super.toArray(generator);
    }

    @Override
    public boolean add(Integer n) {
        if (n % 2 == 0) {
            return internalSet.add(n);
        } else {
            return false;
        }
    }

    @Override
    public boolean remove(Object o) {
        return false;
    }
    @Override
    public boolean addAll(Collection<? extends Integer> c) {
        boolean modified = false;
        for (Integer n : c) {
            if (n % 2 == 0) {
                modified |= internalSet.add(n);
            }
        }
        return modified;
    }

    @Override
    public boolean removeIf(Predicate filter) {
        return Set.super.removeIf(filter);
    }

    @Override
    public void clear() {

    }

    @Override
    public Spliterator spliterator() {
        return Set.super.spliterator();
    }

    @Override
    public Stream stream() {
        return Set.super.stream();
    }

    @Override
    public Stream parallelStream() {
        return Set.super.parallelStream();
    }

    @Override
    public boolean removeAll(Collection c) {
        return false;
    }

    @Override
    public boolean retainAll(Collection c) {
        return false;
    }

    @Override
    public boolean containsAll(Collection c) {
        return false;
    }

    @Override
    public Object[] toArray(Object[] a) {
        return new Object[0];
    }
}
