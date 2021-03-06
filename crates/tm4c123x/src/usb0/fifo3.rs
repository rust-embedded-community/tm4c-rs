#[doc = "Reader of register FIFO3"]
pub type R = crate::R<u32, super::FIFO3>;
#[doc = "Writer for register FIFO3"]
pub type W = crate::W<u32, super::FIFO3>;
#[doc = "Register FIFO3 `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPDATA`"]
pub type EPDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EPDATA`"]
pub struct EPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn epdata(&self) -> EPDATA_R {
        EPDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn epdata(&mut self) -> EPDATA_W {
        EPDATA_W { w: self }
    }
}
