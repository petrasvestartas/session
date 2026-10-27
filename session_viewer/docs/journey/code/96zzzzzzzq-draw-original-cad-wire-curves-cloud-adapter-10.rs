          pass.set_bind_group(1, &draw.group, &[]);
          pass.set_bind_group(2, &draw.address_group, &[]);
          pass.draw(0..6 * draw.total, 0..1);