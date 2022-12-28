#[doc = "Register `ADDOPCRA%s` reader"]
pub struct R(crate::R<ADDOPCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDOPCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDOPCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDOPCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDOPCRA%s` writer"]
pub struct W(crate::W<ADDOPCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDOPCRA_SPEC>;
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
impl From<crate::W<ADDOPCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDOPCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFSEL` reader - Digital Filter Selection"]
pub type DFSEL_R = crate::FieldReader<u8, DFSEL_A>;
#[doc = "Digital Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFSEL_A {
    #[doc = "0: Not use the digital filter"]
    _0X0 = 0,
    #[doc = "1: Use the 1st digital filter"]
    _0X1 = 1,
    #[doc = "2: Use the 2nd digital filter"]
    _0X2 = 2,
    #[doc = "3: Use the 3rd digital filter"]
    _0X3 = 3,
    #[doc = "4: Use the 4th digital filter"]
    _0X4 = 4,
}
impl From<DFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DFSEL_A) -> Self {
        variant as _
    }
}
impl DFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFSEL_A> {
        match self.bits {
            0 => Some(DFSEL_A::_0X0),
            1 => Some(DFSEL_A::_0X1),
            2 => Some(DFSEL_A::_0X2),
            3 => Some(DFSEL_A::_0X3),
            4 => Some(DFSEL_A::_0X4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == DFSEL_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == DFSEL_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == DFSEL_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == DFSEL_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == DFSEL_A::_0X4
    }
}
#[doc = "Field `DFSEL` writer - Digital Filter Selection"]
pub type DFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDOPCRA_SPEC, u8, DFSEL_A, 3, O>;
impl<'a, const O: u8> DFSEL_W<'a, O> {
    #[doc = "Not use the digital filter"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(DFSEL_A::_0X0)
    }
    #[doc = "Use the 1st digital filter"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(DFSEL_A::_0X1)
    }
    #[doc = "Use the 2nd digital filter"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(DFSEL_A::_0X2)
    }
    #[doc = "Use the 3rd digital filter"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(DFSEL_A::_0X3)
    }
    #[doc = "Use the 4th digital filter"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(DFSEL_A::_0X4)
    }
}
#[doc = "Field `GAINSEL` reader - User Gain Table Selection"]
pub type GAINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAINSEL` writer - User Gain Table Selection"]
pub type GAINSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDOPCRA_SPEC, u8, u8, 4, O>;
#[doc = "Field `OFSETSEL` reader - User Offset Table Selection"]
pub type OFSETSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFSETSEL` writer - User Offset Table Selection"]
pub type OFSETSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDOPCRA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Digital Filter Selection"]
    #[inline(always)]
    pub fn dfsel(&self) -> DFSEL_R {
        DFSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:19 - User Gain Table Selection"]
    #[inline(always)]
    pub fn gainsel(&self) -> GAINSEL_R {
        GAINSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Offset Table Selection"]
    #[inline(always)]
    pub fn ofsetsel(&self) -> OFSETSEL_R {
        OFSETSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Digital Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsel(&mut self) -> DFSEL_W<0> {
        DFSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - User Gain Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gainsel(&mut self) -> GAINSEL_W<16> {
        GAINSEL_W::new(self)
    }
    #[doc = "Bits 24:27 - User Offset Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ofsetsel(&mut self) -> OFSETSEL_W<24> {
        OFSETSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Data Operation Control A Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addopcra](index.html) module"]
pub struct ADDOPCRA_SPEC;
impl crate::RegisterSpec for ADDOPCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addopcra::R](R) reader structure"]
impl crate::Readable for ADDOPCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addopcra::W](W) writer structure"]
impl crate::Writable for ADDOPCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDOPCRA%s to value 0"]
impl crate::Resettable for ADDOPCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
