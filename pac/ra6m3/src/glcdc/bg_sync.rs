#[doc = "Register `BG_SYNC` reader"]
pub struct R(crate::R<BG_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BG_SYNC` writer"]
pub struct W(crate::W<BG_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BG_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BG_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP` reader - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK)."]
pub type HP_R = crate::FieldReader<u8, HP_A>;
#[doc = "Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HP_A {
    #[doc = "0: Setting prohibited"]
    _0X0 = 0,
}
impl From<HP_A> for u8 {
    #[inline(always)]
    fn from(variant: HP_A) -> Self {
        variant as _
    }
}
impl HP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HP_A> {
        match self.bits {
            0 => Some(HP_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == HP_A::_0X0
    }
}
#[doc = "Field `HP` writer - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK)."]
pub type HP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_SYNC_SPEC, u8, HP_A, 4, O>;
impl<'a, const O: u8> HP_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(HP_A::_0X0)
    }
}
#[doc = "Field `VP` reader - Background plane vertical synchronization signal assertion position on the basis of line."]
pub type VP_R = crate::FieldReader<u8, VP_A>;
#[doc = "Background plane vertical synchronization signal assertion position on the basis of line.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_A {
    #[doc = "0: Setting prohibited"]
    _0X0 = 0,
}
impl From<VP_A> for u8 {
    #[inline(always)]
    fn from(variant: VP_A) -> Self {
        variant as _
    }
}
impl VP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VP_A> {
        match self.bits {
            0 => Some(VP_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == VP_A::_0X0
    }
}
#[doc = "Field `VP` writer - Background plane vertical synchronization signal assertion position on the basis of line."]
pub type VP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BG_SYNC_SPEC, u8, VP_A, 4, O>;
impl<'a, const O: u8> VP_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(VP_A::_0X0)
    }
}
impl R {
    #[doc = "Bits 0:3 - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn hp(&self) -> HP_R {
        HP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Background plane vertical synchronization signal assertion position on the basis of line."]
    #[inline(always)]
    pub fn vp(&self) -> VP_R {
        VP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    #[must_use]
    pub fn hp(&mut self) -> HP_W<0> {
        HP_W::new(self)
    }
    #[doc = "Bits 16:19 - Background plane vertical synchronization signal assertion position on the basis of line."]
    #[inline(always)]
    #[must_use]
    pub fn vp(&mut self) -> VP_W<16> {
        VP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Plane Setting Synchronization Position Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_sync](index.html) module"]
pub struct BG_SYNC_SPEC;
impl crate::RegisterSpec for BG_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_sync::R](R) reader structure"]
impl crate::Readable for BG_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_sync::W](W) writer structure"]
impl crate::Writable for BG_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BG_SYNC to value 0x0001_0001"]
impl crate::Resettable for BG_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
